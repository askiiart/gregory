use uuid::Uuid;

use crate::errors::Error;
use std::env;
use std::fs::{create_dir_all, File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// Logging for a [`Job`]
// TODO: log to postgres instead; maybe i already made a comment todo-ing this idk
pub(crate) struct JobLogger {
    log_file: File,
    path: String,
}

impl JobLogger {
    pub(crate) fn new(
        data_dir: String,
        job_id: String,
        revision: String,
        run_id: Uuid,
    ) -> JobLogger {
        // get path and create the dir.
        let log_path = format!("{data_dir}/logs/{job_id}/{revision}/{run_id}");
        let log_dir = Path::new(&log_path).parent().unwrap();
        create_dir_all(log_dir).unwrap();

        return JobLogger {
            log_file: OpenOptions::new()
                .create_new(true)
                .append(true)
                .open(&log_path)
                .unwrap(),
            path: log_path,
        };
    }

    /// Log something printed to stdout
    ///
    /// Fun gregory lore: I originally typo'd this as "Strign" and the linter didn't catch it for some reason
    pub(crate) fn stdout(&mut self, text: String, start_time: Instant) -> Result<(), Error> {
        match writeln!(
            &mut self.log_file,
            "[{:.3}] [stdout] {}",
            start_time.elapsed().as_millis() as f64 / 1000.0,
            text
        ) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Log something printed to stderr
    pub(crate) fn stderr(&mut self, text: String, start_time: Instant) -> Result<(), Error> {
        match writeln!(
            &mut self.log_file,
            "[{}] [stderr] {}",
            start_time.elapsed().as_millis() / 1000,
            text
        ) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Returns the path the job's output was logged to
    pub(crate) fn path(&self) -> String {
        return self.path.clone();
    }
}

pub(crate) mod sql {
    use sqlx::{Connection, PgConnection};
    use std::{env, time::Instant};

    /// Returns a new connection to postgres
    ///
    /// *x*: How many times to retry the reconnect
    pub(crate) async fn start(x: u16) -> Box<PgConnection> {
        let mut conn = Box::new(db_connect_with_retries(x).await);
        create_tables(&mut conn).await;
        return conn;
    }

    /// Returns the database environment variables
    ///
    /// Format: (address, username, password)
    pub(crate) fn db_vars() -> (String, String, String) {
        let db_address: String = match env::var("GREGORY_DB_ADDRESS") {
            Ok(address) => address,
            Err(_) => {
                panic!("Environment variable `GREGORY_DB_ADDRESS` not set")
            }
        };
        let db_user: String = match env::var("GREGORY_DB_USER") {
            Ok(user) => user,
            Err(_) => {
                panic!("Environment variable `GREGORY_DB_USER` not set")
            }
        };
        let db_pass: String = match env::var("GREGORY_DB_PASSWORD") {
            Ok(pass) => pass,
            Err(_) => {
                panic!("Environment variable `GREGORY_DB_PASSWORD` not set")
            }
        };

        return (db_address, db_user, db_pass);
    }

    /// Returns the connection to the database
    pub(crate) async fn db_connection() -> Result<PgConnection, sqlx::Error> {
        let (db_address, db_user, db_pass) = db_vars();
        let uri = format!("postgres://{db_user}:{db_pass}@{db_address}/gregory");
        return PgConnection::connect(uri.as_str()).await;
    }

    pub(crate) async fn db_connect_with_retries(x: u16) -> PgConnection {
        let mut conn = db_connection().await;
        if conn.is_ok() {
            return conn.unwrap();
        }

        for _ in 0..x {
            conn = db_connection().await;
            if conn.is_ok() {
                break;
            }
        }

        return conn.unwrap();
    }

    // TODO: when adding logging to postgres directly, update this so it 1) adds the job at the start, 2) logs line-by-line, and 3) adds the end time and exit code at the end of the job
    pub(crate) async fn log_job(
        mut conn: Box<PgConnection>,
        start_time: Instant,
        end_time: Instant,
        exit_code: Option<i32>,
        job_id: String,
        revision: String,
        uuid: String,
        log_path: String,
    ) {
        let start_time =
            chrono::DateTime::from_timestamp_millis(start_time.elapsed().as_millis() as i64)
                .unwrap()
                .format("%+")
                .to_string();
        let end_time =
            chrono::DateTime::from_timestamp_millis(end_time.elapsed().as_millis() as i64)
                .unwrap()
                .format("%+")
                .to_string();
        let exit_code = match exit_code {
            Some(code) => code.to_string(),
            None => "NULL".to_string(),
        };
        sqlx::query(format!("INSERT INTO job_logs (start_time, end_time, exit_code, job_id, revision, uuid, log_path)
                VALUES ('{start_time}', '{end_time}', {exit_code}, '{job_id}', '{revision}', '{uuid}', '{log_path}'));            
").as_str()).execute(conn.as_mut()).await.unwrap();
    }

    /// Tries to connect to the database *x* times, panics after reaching that limit

    /// Creates table(s) for gregory if they don't exist already
    pub(crate) async fn create_tables(conn: &mut Box<PgConnection>) {
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS job_logs (
    start_time   timestamp,
    end_time    timestamp,
    duration    interval GENERATED ALWAYS AS (end_time - start_time) STORED,
    exit_code    smallint,
    job_id     text,
    revision    text,
    uuid      text,
    container_name  text GENERATED ALWAYS AS (job_id || '-' || uuid) STORED,
    log_path        text
);
",
        )
        .execute(conn.as_mut())
        .await
        .unwrap();
    }
}

#[test]
pub(crate) fn test_db_vars() {
    assert_eq!(
        (
            "postgres".to_string(),
            "gregory".to_string(),
            "pass".to_string()
        ),
        sql::db_vars()
    )
}
