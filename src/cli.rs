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
        #[arg(short, long, default_value = "gregory")]
        binary_name: String,
    },
    ///Runs it
    Run {
        ///Path to the config file
        #[arg(short, long, default_value = "gregory.yml")]
        config: String,
        /* Not yet supported
        #[arg(short, long)]
        daemonize: bool,
        */
    },
}

#[derive(Subcommand, Debug)]
pub enum ShellCommands {
    Bash,
    Zsh,
    Fish,
    Elvish,
    Powershell,
}
