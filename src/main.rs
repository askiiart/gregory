use crate::cli::*;
use crate::data::*;
use alphanumeric_sort::sort_str_slice;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use std::io::stdout;

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

    println!("{:?}", config);
}
