//! Data structs. used by gregory and stuff for handling them

use crate::errors::Error;
use serde::Deserialize;
use std::time;
use std::{collections::HashMap, fs, thread};

/// The config for gregory
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    /// Maximum number of jobs to run simultaneously
    #[serde(default = "max_jobs", rename = "max-jobs")]
    pub(crate) max_jobs: u32,
    /// Maximum number of threads to use
    #[serde(default = "max_threads", rename = "max-threads")]
    pub(crate) max_threads: f32,
    #[serde(default = "data", rename = "data-dir")]
    pub(crate) data_dir: String,
    /// Holds the packages, including their compilation and packaging
    ///
    /// See config reference in the docs for details.
    ///
    /// See [`Package`] for details
    pub(crate) packages: HashMap<String, Package>,
    /// The jobs for updating the repo, organized by distro/repo name
    #[serde(rename = "update-repo")]
    pub(crate) update_repo: HashMap<String, Job>,
    /// All volumes, organized like this:
    ///
    /// Format: `librewolf = "./data/librewolf:/librewolf"` - like Docker/Podman formatting
    #[serde(default = "volumes")]
    pub(crate) volumes: HashMap<String, String>,
}

impl Config {
    pub(crate) fn from_file(filename: String) -> Result<Config, Error> {
        match fs::read_to_string(filename) {
            Ok(raw_data) => match toml::from_str(raw_data.as_str()) {
                Ok(conf) => return Ok(conf),
                Err(e) => {
                    return Err(Error::DeserError(e));
                }
            },
            Err(e) => {
                return Err(Error::IOError(e));
            }
        }
    }
}

/// Holds the data for a job
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Job {
    /// What revision of the job config, temporary until better revision tracking is added
    #[serde(default = "revision")]
    pub(crate) revision: String,
    /// How many threads to limit this job to; recommended to set it to the max threads the job will use
    ///
    /// If `threads` isn't specified, it will fall back to `max_threads` (from [`Config`]); the same behavior applies if `threads` is greater than `max_threads`
    #[serde(default = "job_threads")]
    pub(crate) threads: f32,
    /// The OCI image to run it in
    ///
    /// For example, `docker.io/library/debian:latest`
    pub(crate) image: String,
    /// The commands to run in the job
    pub(crate) commands: Vec<String>,
    /// A list of all volumes given their name - see [`Config`] -> `volumes`
    pub(crate) volumes: Option<Vec<String>>,
    /// Whether the job W be privileged
    ///
    /// Defauolt: false
    #[serde(default = "privileged")]
    pub(crate) privileged: bool,
    #[serde(default = "shell")]
    pub(crate) shell: String,
}

/// Holds the data for a certain package's config
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Package {
    /// What other packages gregory handles which this depends on
    #[serde(default = "dependencies")]
    pub(crate) dependencies: Vec<String>,
    /// The compilation [`Job`] - optional
    pub(crate) compilation: Option<Job>,
    /// The packaging [`Job`]s, organized by the distro/repo name
    pub(crate) packaging: HashMap<String, Job>,
}

/// The exit status and stuff for a [`Job`]
#[derive(Debug, Clone)]
pub(crate) struct JobExitStatus {
    /// The [`Job`] this status is from
    pub(crate) job: Job,
    /// The status code returned by the command - note that this can be None if the program exits due to a signal like SIGKILL.
    ///
    /// This is stored as a u16 rather than a u8 so that 65535 can be returned if there is no exit code rather than doing an Option or something, which I fear will probably come back to haunt me, but whatever
    /// Update: I knew it. Why did I do this. Anyways this is gonna be an Option<i32> like Command uses now
    ///
    /// Hell this isn't even coming back to haunt me for any sane reason, it's because I went with the actually sensible decision of Option<i32> in better-commands, so if I want to use that then I'm stuck using this.
    ///
    /// Anyways I'll stop rambling now.
    pub(crate) exit_code: Option<i32>,
    /// Where the log is
    ///
    /// TEMPORARY
    pub(crate) log_path: String,
    /// How long it took to run the job
    pub(crate) duration: time::Duration,
    /// The name of the container this job ran in
    pub(crate) container_name: String,
    /// Uuid
    pub(crate) job_uuid: String,
}

// ==========================
// ===                    ===
// ===    ↓ DEFAULTS ↓    ===
// ===                    ===
// ==========================

/// Returns the default number of max threads.
pub(crate) fn max_threads() -> f32 {
    let total_threads = thread::available_parallelism().unwrap().get() as f32;
    if total_threads >= 32.0 {
        return total_threads - 4.0;
    } else if total_threads >= 12.0 {
        return total_threads - 2.0;
    } else if total_threads >= 3.0 {
        return total_threads - 1.0;
    } else {
        return total_threads;
    }
}

/// Returns the default number of max jobs - 1
pub(crate) fn max_jobs() -> u32 {
    return 1;
}

/// Returns the default volumes, i.e. none
pub(crate) fn volumes() -> HashMap<String, String> {
    return HashMap::new();
}

/// Returns the default number of threads for a job - [`max_threads()`]
pub(crate) fn job_threads() -> f32 {
    return max_threads();
}

/// Default (false) for whether a job should be privileged
pub(crate) fn privileged() -> bool {
    return false;
}

/// Default (`/bin/sh`) for which shell to use
pub(crate) fn shell() -> String {
    return "/bin/sh".to_string();
}

/// Default id (`-1`)
pub(crate) fn id() -> String {
    return "-1".to_string();
}

/// Default revision (`1`)
pub(crate) fn revision() -> String {
    return "1".to_string();
}

pub(crate) fn data() -> String {
    return "./data".to_string();
}

pub(crate) fn dependencies() -> Vec<String> {
    return Vec::new();
}
