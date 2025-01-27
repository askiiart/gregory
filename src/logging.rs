use crate::errors::Error;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::Instant;

/// Logging for a [`Job`]
pub(crate) struct JobLogger {
    log_file: File,
}

impl JobLogger {
    pub(crate) fn new(path: String) -> Result<JobLogger, Error> {
        match OpenOptions::new().create_new(true).append(true).open(path) {
            Ok(f) => return Ok(JobLogger { log_file: f }),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Log something printed to stdout
    ///
    /// Fun gregory lore: I originally typo'd this as "Strign" and the linter didn't catch it for some reason
    pub(crate) fn stdout(&mut self, text: String) -> Result<(), Error> {
        match writeln!(&mut self.log_file, "[stdout] {}", text) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Log something printed to stderr
    pub(crate) fn stderr(&mut self, text: String) -> Result<(), Error> {
        match writeln!(&mut self.log_file, "[stderr] {}", text) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }
}
