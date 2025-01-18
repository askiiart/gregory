use crate::cli::*;
use crate::data::*;
use alphanumeric_sort::sort_str_slice;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use std::fs;
use std::fs::create_dir_all;
use std::fs::remove_dir_all;
use std::fs::write;
use std::fs::File;
use std::io::stdout;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::process::Command;
use std::time::Instant;
use uuid::Uuid;
use better_commands;

mod cli;
mod data;
mod errors;
mod tests;
mod logging;

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
    let config = config_from_file(config_path).unwrap();

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
    let mut threads = job.threads;
    if job.threads > conf.max_threads {
        threads = conf.max_threads;
    }

    let container_name: String = format!("gregory-{}-{}-{}", job.id, job.revision, Uuid::now_v7());

    // create log path
    let log_path = &format!("{}/logs/{container_name}", conf.data_dir); // can't select fields in the format!() {} thing, have to do this
    let log_dir: &Path = Path::new(log_path).parent().unwrap();
    create_dir_all(log_dir).unwrap();

    // write the script
    let script_path = &format!("{}/tmp/{container_name}.sh", conf.data_dir); // can't select fields in the format!() {} thing, have to do this
                                                                             // create dir for the script
    let script_dir: &Path = Path::new(script_path).parent().unwrap();
    create_dir_all(script_dir).unwrap();
    write(
        script_path,
        job.commands
            //.iter()
            //.map(|item| {
            //    // TODO: FIGURE OUT HOW TO HANDLE IT ESCAPING IT OR WHATEVER AAAAAAAAAAAAA
            //    // update: i have no idea what i was talking about previously
            //})
            //.collect::<Vec<String>>()
            .join("\n"),
    )
    .unwrap();

    // set permissions - *unix specific*
    let mut perms = File::open(script_path)
        .unwrap()
        .metadata()
        .unwrap()
        .permissions();
    PermissionsExt::set_mode(&mut perms, 0o755);

    let mut cmd_args: Vec<String> = vec![
        "run".to_string(),
        format!("--name={container_name}"),
        format!("--cpus={threads}"),
        format!("--privileged={}", job.privileged),
        format!("-v={script_path}:/gregory-entrypoint.sh"),
    ];
    for vol in job.clone().volumes.unwrap_or(Vec::new()) {
        match conf.volumes.get(&vol) {
            Some(item) => {
                cmd_args.push(format!("-v={}", item));
            }
            None => {
                println!()
            }
        }
    }
    cmd_args.push(format!(
        "--entrypoint=[\"{}\", \"/gregory-entrypoint.sh\"]",
        &job.shell
    ));
    cmd_args.push(job.clone().image);
    // TODO: TEMPORARY - update to actually write it in the future
    let cmd_output = better_commands::run_funcs(Command::new("podman").args(cmd_args),  {
        |stdout_lines|
        for line in stdout_lines {
            println!("[stdout] {}", line.unwrap());
        }
    },
{
    |stderr_lines|
    for line in stderr_lines {
        println!("[stderr] {}", line.unwrap());
    }
});
    // remove tmp dir
    remove_dir_all(script_dir).unwrap();

    println!("{:?}", cmd_output);

    return JobExitStatus {
        container_name: container_name,
        duration: cmd_output.clone().duration(),
        job: job,
        exit_code: cmd_output.status_code(),
        log_path: log_path.clone(),
    };
}
