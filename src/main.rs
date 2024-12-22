use crate::cli::*;
use alphanumeric_sort::sort_str_slice;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Elvish, Fish, PowerShell, Zsh};
use std::fs;
use std::io::stdout;
use yaml_rust2::YamlLoader;
mod cli;
mod tests;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenCompletion { shell } => match shell {
            ShellCommands::Bash { binary_name } => {
                generate(Bash, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Zsh { binary_name } => {
                generate(Zsh, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Fish { binary_name } => {
                generate(Fish, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::Elvish { binary_name } => {
                generate(Elvish, &mut Cli::command(), binary_name, &mut stdout());
            }
            ShellCommands::PowerShell { binary_name } => {
                generate(PowerShell, &mut Cli::command(), binary_name, &mut stdout());
            }
        },
        Commands::Run { config, daemonize } => {
            println!("{}", config);
            println!("{}", daemonize)
        }
    }
}

fn run(config_path: String) {
    let tmp = fs::read_to_string(config_path.as_str()).unwrap();
    let config = YamlLoader::load_from_str(tmp.as_str()).unwrap()[0].clone();
    println!("{:?}", config)
}
