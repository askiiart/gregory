use crate::cli::Commands;
use crate::cli::*;
use alphanumeric_sort::sort_str_slice;
use clap::{CommandFactory, Parser};
use clap_complete::aot::{generate, Bash, Fish, Zsh};
use std::fs;
use std::io::stdout;
use yaml_rust2::YamlLoader;
mod cli;
mod tests;

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::GenerateBashCompletions => {
            generate(
                Bash,
                &mut Cli::command(),
                "gregory",
                &mut stdout(),
            );
        }
        Commands::GenerateZshCompletions => {
            generate(
                Zsh,
                &mut Cli::command(),
                "gregory",
                &mut stdout(),
            );
        }
        Commands::GenerateFishCompletions => {
            generate(
                Fish,
                &mut Cli::command(),
                "gregory",
                &mut stdout(),
            );
        }
        Commands::Run => {}
    }
}

fn run(config_path: String) {
    let tmp = fs::read_to_string(config_path.as_str()).unwrap();
    let config = YamlLoader::load_from_str(tmp.as_str()).unwrap()[0].clone();
    println!("{:?}", config)
}
