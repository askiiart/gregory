use crate::cli::*;
use crate::data::*;
use better_commands;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use logging::sql;
use sqlx::PgConnection;
use std::collections::HashMap;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use std::fs::write;
use std::fs::File;
use std::io::stdout;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;
use std::time::SystemTime;
use uuid::Uuid;

mod cli;
mod data;
mod errors;
mod logging;
mod tests;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenCompletion { shell, binary_name } => match shell {
            ShellCommands::Bash => {
                generate(Bash, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Zsh => {
                generate(Zsh, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Fish => {
                generate(Fish, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Elvish => {
                generate(Elvish, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Powershell => {
                generate(PowerShell, &mut Cli::command(), binary_name, &mut stdout());
            }
        },
        Commands::Run { config } => {
            run(config).await;
        }
    }
}

async fn run(config_path: String) {
    let config = Config::from_file(config_path).unwrap(); // this reads the file to a [`Config`] thing
    let state = State::from_config(config.clone()).await;

    // TODO: improve efficiency of all this logic
    // TODO: Also clean it up and split it into different functions, especially the job sorter
    // TODO: figure all this out and stuff and update the comments above this - the dependency map is done though

    let mut ordered: Vec<String> = Vec::new(); // holds the job ids in order of how they should be run

    let mut update_repo_jobs: HashMap<String, Job> = HashMap::new();
    for (repo, job) in config.clone().update_repo {
        update_repo_jobs.insert(format!("update-repo.{}", repo), job);
    }

    // TODO: Add logic to add repo update repos when relevant (see dependencies) here - or maybe do that logic earlier?

    let failed_packages: Vec<String> = Vec::new();

    let mut pg_connection = sql::start(5).await;

    // runs the jobs (will need to be updated after sorting is added)
    for (job_id, job) in state.jobs {
        println!("Running {job_id}");
        let start_time = SystemTime::now();
        let job_exit_status = run_job(&state.conf, job_id.clone(), job.clone());
        match job_exit_status.exit_code.clone() {
            Some(e) => {
                println!(" Job completed, exit code {e}");
            }
            None => {
                println!(" Job completed, !!! no exit code !!!");
                println!("  This means the process was terminated by a signal, like SIGKILL, which you should probably look into. See also: https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.code")
            }
        }

        sql::log_job(
            &mut pg_connection,
            start_time,
            start_time + job_exit_status.duration,
            job_exit_status.exit_code,
            job_id,
            job.revision,
            job_exit_status.job_uuid,
            job_exit_status.log_path.clone(),
        )
        .await;

        println!(
            " Logged metadata to postgres database; log file at {}",
            job_exit_status.log_path
        );

        println!()
    }

    // run repo updates
    for (job_id, job) in update_repo_jobs {
        println!("Running {job_id}");
        let start_time = SystemTime::now();
        let job_exit_status = run_job(&state.conf, job_id.clone(), job.clone());
        match job_exit_status.exit_code.clone() {
            Some(e) => {
                println!(" Job completed, exit code {e}");
            }
            None => {
                println!(" Job completed, !!! no exit code !!!");
                println!("  This means the process was terminated by a signal, like SIGKILL, which you should probably look into. See also: https://doc.rust-lang.org/std/process/struct.ExitStatus.html#method.code")
            }
        }

        sql::log_job(
            &mut pg_connection,
            start_time,
            start_time + job_exit_status.duration,
            job_exit_status.exit_code,
            job_id,
            job.revision,
            job_exit_status.job_uuid,
            job_exit_status.log_path.clone(),
        )
        .await;

        println!(
            " Logged metadata to postgres database; log file at {}",
            job_exit_status.log_path
        );

        println!()
    }
}

fn run_job(conf: &Config, job_id: String, job: Job) -> JobExitStatus {
    // limit threads to max_threads in the config
    let mut threads = job.threads;
    if job.threads > conf.max_threads {
        threads = conf.max_threads;
    }

    let run_id = Uuid::now_v7();

    let job_logger = Arc::new(Mutex::new(logging::JobLogger::new(
        conf.data_dir.clone(),
        job_id.clone(),
        job.revision.clone(),
        run_id,
    )));

    // write the script
    let script_path: String = format!("{}/tmp/{}.sh", conf.data_dir, run_id);
    let script_dir = Path::new(&script_path).parent().unwrap(); // create dir for the script
    create_dir_all(script_dir).unwrap();
    write(&script_path, job.commands.join("\n")).unwrap();

    // set permissions - *unix specific*
    let mut perms = File::open(&script_path)
        .unwrap()
        .metadata()
        .unwrap()
        .permissions();
    PermissionsExt::set_mode(&mut perms, 0o755);

    // run the job
    let mut cmd_args: Vec<String> = vec![
        "run".to_string(),
        "--rm".to_string(),
        format!("--name={job_id}-{run_id}"),
        format!("--cpus={threads}"),
        format!("--privileged={}", job.privileged),
        format!("-v={script_path}:/gregory-entrypoint.sh"),
    ];
    for vol in job.clone().volumes.unwrap_or(Vec::new()) {
        match conf.volumes.get(&vol) {
            Some(item) => {
                cmd_args.push(format!("-v={}", item));
            }
            None => {}
        }
    }
    cmd_args.push(format!(
        "--entrypoint=[\"{}\", \"/gregory-entrypoint.sh\"]",
        &job.shell
    ));
    cmd_args.push(job.clone().image);

    let cmd_output = better_commands::run_funcs(
        Command::new("podman").args(cmd_args),
        {
            let start_time = Instant::now();
            let logger_clone = Arc::clone(&job_logger);
            move |stdout_lines| {
                for line in stdout_lines {
                    let _ = logger_clone
                        .lock()
                        .unwrap()
                        .stdout(line.unwrap(), start_time);
                }
            }
        },
        {
            let start_time = Instant::now();
            let logger_clone = Arc::clone(&job_logger);
            move |stderr_lines| {
                for line in stderr_lines {
                    let _ = logger_clone
                        .lock()
                        .unwrap()
                        .stderr(line.unwrap(), start_time);
                }
            }
        },
    );

    // remove tmp dir/clean up
    remove_dir_all(script_dir).unwrap();

    let log_path = job_logger.lock().unwrap().path();

    return JobExitStatus {
        container_name: script_path,
        duration: cmd_output.clone().duration(),
        job,
        exit_code: cmd_output.status_code(),
        log_path,
        job_uuid: run_id.to_string(),
    };
}

/// Turns a job name into the relevant data - (category  (i.e. "packaging"), package name (i.e. "librewolf"), name (i.e. "compilation"))
fn job_id_to_metadata(job_id: String) -> (String, String, String) {
    let data = job_id
        .split(".")
        .map(|item| item.to_string())
        .collect::<Vec<String>>();
    return (data[0].clone(), data[1].clone(), data[2].clone());
}

/// Returns all the dependencies for a package recursively, *not* including the package's own jobs (e.g. compilation)
fn recursive_deps_for_package(package_name: String, conf: Config) -> Vec<String> {
    let mut deps: Vec<String> = Vec::new();

    for dep_name in conf
        .packages
        .get(&package_name)
        .unwrap()
        .dependencies
        .clone()
    {
        // add recursive dependencies
        deps.append(&mut recursive_deps_for_package(
            dep_name.clone(),
            conf.clone(),
        ));
    }
    // add its compilation to deps
    match conf
        .packages
        .get(&package_name)
        .unwrap()
        .compilation
        .clone()
    {
        Some(_) => {
            deps.push(format!("packages.{package_name}.compilation"));
        }
        None => {}
    }

    // add packaging jobs to deps
    for (packaging_job_name, _) in conf.packages.get(&package_name).unwrap().packaging.clone() {
        deps.push(format!(
            "packages.{package_name}.packaging.{packaging_job_name}"
        ))
    }

    return deps;
}

struct State {
    /// The entire config, from the config file.
    conf: Config,
    /// A hashmap mapping all job ids to what jobs depend on them (recursively)
    ///
    /// Using the example config (`gregory.example.toml`):
    ///
    /// ```json
    /// {
    ///     "packages.some-librewolf-dependency.packaging.fedora": [
    ///         "packages.librewolf.compilation",
    ///         "packages.librewolf.packaging.fedora",
    ///     ],
    ///     "packages.some-librewolf-dependency.compilation": [
    ///         "packages.librewolf.compilation",
    ///         "packages.librewolf.packaging.fedora",
    ///         "packages.some-librewolf-dependency.packaging.fedora",
    ///     ],
    ///     "packages.librewolf.compilation": [
    ///         "packages.librewolf.packaging.fedora",
    ///     ],
    /// }
    /// ```
    dependency_map: HashMap<String, Vec<String>>,
    /// A hashmap mapping all job ids to their jobs
    jobs: HashMap<String, Job>,
}

impl State {
    pub(crate) async fn from_file(filename: String) -> State {
        let conf = Config::from_file(filename).unwrap();
        return State::from_config(conf).await;
    }

    pub(crate) async fn from_config(conf: Config) -> State {
        let mut jobs = HashMap::new();

        for (package_name, package) in conf.clone().packages {
            match package.compilation {
                Some(tmp) => {
                    jobs.insert(format!("packages.{}.compilation", package_name), tmp);
                }
                None => {}
            }

            for (job_name, job) in package.packaging {
                jobs.insert(
                    format!("packages.{}.packaging.{}", package_name, job_name),
                    job,
                );
            }
        }

        return State {
            conf: conf.clone(),
            jobs: jobs.clone(),
            dependency_map: State::dependency_map(jobs, conf),
        };
    }

    /// Returns a hashmap mapping all job ids to what jobs depend on them (recursively)
    ///
    /// Example output using the example toml:
    ///
    /// ```json
    /// {
    ///     "packages.some-librewolf-dependency.compilation": [
    ///         "packages.librewolf.compilation",
    ///         "packages.librewolf.packaging.fedora",
    ///         "packages.some-librewolf-dependency.packaging.fedora",
    ///     ],
    ///     "packages.librewolf.packaging.fedora": [],
    ///     "packages.librewolf.compilation": [
    ///         "packages.librewolf.packaging.fedora",
    ///     ],
    ///     "packages.some-librewolf-dependency.packaging.fedora": [
    ///         "packages.librewolf.compilation",
    ///         "packages.librewolf.packaging.fedora",
    ///     ],
    /// }
    /// ```
    fn dependency_map(jobs: HashMap<String, Job>, conf: Config) -> HashMap<String, Vec<String>> {
        let mut dep_map: HashMap<String, Vec<String>> = HashMap::new(); // holds job ids and every job they depend on (recursively) - not just specified dependencies, also packaging depending on compilation

        for (job_id, _) in jobs.clone() {
            dep_map.insert(job_id, Vec::new());
        }

        for (job_id, _) in jobs.clone() {
            let (_, package_name, _) = job_id_to_metadata(job_id.clone());

            for dep_name in conf
                .packages
                .get(&package_name)
                .unwrap()
                .dependencies
                .clone()
            {
                let all_deps = recursive_deps_for_package(dep_name.clone(), conf.clone());
                for dep in all_deps {
                    dep_map.get_mut(&dep).unwrap().push(job_id.clone());
                }
            }
        }

        // add compilation jobs when relevant
        for (package_name, package) in conf.packages {
            if package.compilation.is_some() {
                if !dep_map.contains_key(&format!("packages.{package_name}.compilation")) {
                    dep_map.insert(format!("packages.{package_name}.compilation"), Vec::new());
                }

                for (job_name, _) in package.packaging {
                    dep_map
                        .get_mut(&format!("packages.{package_name}.compilation"))
                        .unwrap()
                        .push(format!("packages.{package_name}.packaging.{job_name}"));
                }
            }
        }

        // deduplicate dependencies
        for (_, deps) in dep_map.iter_mut() {
            deps.dedup();
        }

        return dep_map;
    }
}
