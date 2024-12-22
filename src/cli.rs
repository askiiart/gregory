use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Generate shell completions
    GenCompletion {
        #[command(subcommand)]
        shell: ShellCommands,
    },
    ///Runs it
    Run {
        #[arg(short, long)]
        config: String,
        #[arg(short, long)]
        daemonize: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum ShellCommands {
    Bash {
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
    Zsh {
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
    Fish {
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
    Elvish {
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
    PowerShell {
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
}
