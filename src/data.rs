//! Data structs. used by gregory and stuff for handling them

use serde::Deserialize;
use std::time;
use std::{collections::HashMap, fs, thread};

/// The config for gregory
#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Config {
    /// What level to log at
    ///
    /// - 0: Error
    /// - 1: Warning
    /// - 2: Info
    /// - 3: Debug
    #[serde(default = "log_level", rename = "log-level")]
    // the rename lets it use `log-level` instead in the toml file - this is not an alias, `log_level` in the toml will *not* work
    pub(crate) log_level: u8,
    /// Maximum number of jobs to run simultaneously
    #[serde(default = "max_jobs", rename = "max-jobs")]
    pub(crate) max_jobs: u32,
    /// Maximum number of threads to use
    #[serde(default = "max_threads", rename = "max-threads")]
    pub(crate) max_threads: u32,
    #[serde(default = "data", rename = "data-dir")]
    pub(crate) data_dir: String,
    /// Holds the packages, including their compilation and packaging
    ///
    /// Format: `{ "librewolf": Package { compilation, packaging } }`
    ///
    /// See [`Package`] for details
    pub(crate) packages: HashMap<String, Package>,
    /// The jobs for updating the repo, organized by distro/repo name
    #[serde(rename = "update-repo")]
    pub(crate) update_repo: HashMap<String, Job>,
    /// All volumes, organized like this:
    ///
    /// Format: `{ "librewolf": "./data/librewolf:/librewolf" }` - like Docker/Podman formatting
    #[serde(default = "volumes")]
    pub(crate) volumes: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Job {
    /// An ID to identify the job, such as the compilation of a program
    #[serde(default = "id")]
    pub(crate) id: String,
    #[serde(default = "revision")]
    pub(crate) revision: String,
    /// How many threads to limit this job to; recommended to set it to the max threads the job will use
    ///
    /// If `threads` isn't specified, it will fall back to `max_threads` (from [`Config`]); the same behavior applies if `threads` is greater than `max_threads`
    #[serde(default = "job_threads")]
    pub(crate) threads: u32,
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
    /// The compilation [`Job`] - optional
    pub(crate) compilation: Option<Job>,
    /// The packaging [`Job`]s, organized by the distro/repo name
    pub(crate) packaging: HashMap<String, Job>,
}

/// The exit status and stuff for a [`Job`]
#[derive(Debug, Clone)]
pub(crate) struct JobExitStatus {
    pub(crate) job: Job,
    /// The [`Job`] this status is from
    /// 
    /// This is stored as a u16 rather than a u8 so that 65535 can be returned if there is no exit code rather than doing an Option or something, which I fear will probably come back to haunt me, but whatever
    pub(crate) exit_code: u16,
    /// Where the log is
    ///
    /// TEMPORARY
    /// TODO: Have main() handle logs and writing them to the database, not doing it in run_job()
    pub(crate) log_path: String,
    /// How long it took to run the job
    pub(crate) duration: time::Duration,
    /// The name of the container this job ran in
    pub(crate) container_name: String,
}

pub(crate) fn config_from_file(filename: String) -> Config {
    let toml: Config = toml::from_str(fs::read_to_string(filename).unwrap().as_str()).unwrap();
    return toml;
}

// ==========================
// ===                    ===
// ===    ↓ DEFAULTS ↓    ===
// ===                    ===
// ==========================

/// Returns the default log level (1 - warning)
pub(crate) fn log_level() -> u8 {
    return 1;
}

/// Returns the default number of max threads
pub(crate) fn max_threads() -> u32 {
    let total_threads = thread::available_parallelism().unwrap().get() as u32;
    if total_threads >= 32 {
        return total_threads - 4;
    } else if total_threads >= 12 {
        return total_threads - 2;
    } else if total_threads >= 3 {
        return total_threads - 1;
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
pub(crate) fn job_threads() -> u32 {
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
