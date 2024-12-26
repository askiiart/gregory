//! Datasets used by gregory and stuff for handling them

use serde::Deserialize;
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
    log_level: u8,
    /// Maximum number of jobs to run simultaneously
    #[serde(default = "max_jobs", rename = "max-jobs")]
    max_jobs: u32,
    /// Maximum number of threads to use
    #[serde(default = "max_threads", rename = "max-threads")]
    max_threads: u32,
    /// Holds the packages, including their compilation and packaging
    /// 
    /// Format: { "librewolf": Package { compilation, packaging } }
    /// 
    /// See [`Package`] for details
    packages: HashMap<String, Package>,
    #[serde(rename = "update-repo")]
    update_repo: HashMap<String, Job>,
    #[serde(default = "volumes")]
    volumes: HashMap<String, String>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Job {
    #[serde(default = "job_threads")]
    /// How many threads to limit this job to; recommended to set it to the max threads the job will use
    /// 
    /// If `threads` isn't specified, it will fall back to `max_threads` (from [`Config`]); the same behavior applies if `threads` is greater than `max_threads`
    threads: u32,
    image: String,
    commands: Vec<String>,
    volumes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Deserialize)]
pub(crate) struct Package {
    compilation: Option<Job>,
    packaging: HashMap<String, Job>,
}

pub(crate) fn config_from_file(filename: String) -> Config {
    let yaml: Config = serde_yml::from_str(fs::read_to_string(filename).unwrap().as_str()).unwrap();
    return yaml;
}

// ==========================
// ===                    ===
// ===    ↓ DEFAULTS ↓    ===
// ===                    ===
// ==========================

pub(crate) fn log_level() -> u8 {
    return 1;
}

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

pub(crate) fn max_jobs() -> u32 {
    return 1;
}

pub(crate) fn volumes() -> HashMap<String, String> {
    return HashMap::new();
}

pub(crate) fn job_threads() -> u32 {
    return max_threads();
}
