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

    #[clap(alias = "d", alias = "down")]
    Download(commands::download::DownloadArgs),

    #[clap(alias = "e")]
    Exists(commands::exists::ExistsArgs),

    #[clap(alias = "del")]
    Delete(commands::delete::DeleteArgs),
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload(args) => commands::upload::upload_file_cmd(args),
        Commands::Download(args) => commands::download::download_file_cmd(args),
        Commands::Exists(args) => commands::exists::exists_cmd(args),
        Commands::Delete(args) => commands::delete::delete_cmd(args),
    }
}
