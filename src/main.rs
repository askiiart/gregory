use crate::cli::*;
use crate::data::*;
use alphanumeric_sort::sort_str_slice;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use std::fs;
use std::fs::create_dir_all;
use std::fs::write;
use std::fs::File;
use std::io::stdout;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use uuid::Uuid;

mod cli;
mod data;
mod tests;

fn main() {
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
            println!("Config path: {}", config);
            run(config);
        }
    }
}

fn run(config_path: String) {
    let config = config_from_file(config_path);

    println!("{:#?}", config);

    let mut jobs: Vec<Job> = Vec::new();

    for (_, package) in config.clone().packages {
        match package.compilation {
            Some(tmp) => {
                jobs.push(tmp);
            }
            None => {}
        }

        for (_, job) in package.packaging {
            jobs.push(job);
        }
    }

    for (_, job) in config.clone().update_repo {
        jobs.push(job);
    }

    for job in jobs {
        println!("{:#?}", run_job(config.clone(), job));
    }
}

fn run_job(conf: Config, job: Job) -> JobExitStatus {
    // limit threads to max_threads in the config
    let mut threads: u32 = job.threads;
    if job.threads > conf.max_threads {
        threads = conf.max_threads;
    }

    let container_name: String = format!("gregory-{}-{}-{}", job.id, job.revision, Uuid::now_v7());

    let log_path = &format!("{}/logs/{container_name}.txt", conf.data_dir); // can't select fields in the format!() {} thing, have to do this
    let log_dir: &Path = Path::new(log_path);
    create_dir_all(log_dir.parent().unwrap()).unwrap();
    write(log_path, job.commands.join("\n")).unwrap();

    // set permissions - *unix specific*
    let mut perms = File::open(log_path)
        .unwrap()
        .metadata()
        .unwrap()
        .permissions();
    PermissionsExt::set_mode(&mut perms, 0o755);

    let now = Instant::now();
    let cmd_args: Vec<String> = vec![
        "run".to_string(),
        format!("--name={container_name}"),
        format!("--cpus={threads}"),
        format!("--privileged={}", job.privileged),
        format!("-v={log_path}:/gregory-entrypoint.sh"),
        format!("--entrypoint=['{}', '/gregory-entrypoint.sh']", &job.shell),
        job.clone().image
    ];
    let cmd_output = Command::new("podman").args(cmd_args).output().unwrap();
    let elapsed = now.elapsed();


    println!("{:?}", cmd_output);

    return JobExitStatus {
        container_name: container_name,
        duration: elapsed,
        job: job,
        exit_code: cmd_output.status.code().ok_or_else(|| 65535).unwrap() as u16,
        log_path: log_path.clone(),
    };
}
