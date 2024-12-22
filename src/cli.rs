use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    ///Generate bash completions
    GenerateBashCompletions,
    ///Generate zsh completions
    GenerateZshCompletions,
    ///Generate fish completions
    GenerateFishCompletions,
    ///Runs it
    Run {
        #[arg(short, long)]
        config: String,
    },
}
