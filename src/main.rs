mod commands;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "ffsend", version = "0.1", author = "al", about = "A CLI for file sharing using ffsend")]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[clap(alias = "u", alias = "up")]
    Upload(commands::upload::UploadArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload(args) => {
            commands::upload::upload_file_cmd(args);
        }
    }
}