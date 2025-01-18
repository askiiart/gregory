use crate::errors::Error;
use std::io::Write;
use std::{
    fs::{File, OpenOptions},
    os::unix::fs::FileExt,
};

/// The logger for gregory itself - NOT for jobs
pub(crate) struct Logger {
    log_file: File,
}

impl Logger {
    pub(crate) fn new(path: String) -> Result<Logger, Error> {
        match OpenOptions::new().append(true).open(path) {
            Ok(f) => return Ok(Logger { log_file: f }),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Log a warning
    ///
    /// Fun gregory lore: I originally typo'd this as "Strign" and the linter didn't catch it for some reason
    pub(crate) fn warning(&mut self, text: String) -> Result<(), Error> {
        match writeln!(&mut self.log_file, "[WARNING] {}", text) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }

    /// Log an error
    pub(crate) fn error(&mut self, text: String) -> Result<(), Error> {
        match writeln!(&mut self.log_file, "[ERROR] {}", text) {
            Ok(_) => return Ok(()),
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }
}
