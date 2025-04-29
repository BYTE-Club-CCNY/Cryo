mod commands;

use clap::{CommandFactory, Parser, Subcommand};

#[derive(Parser)]
#[clap(
    name = "ffsend",
    version = "0.1",
    author = "al",
    about = "A CLI for file sharing using ffsend"
)]
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

    #[clap(alias = "i")]
    Info(commands::info::InfoArgs),

    /// Display help information
    Help {
        /// Subcommand to display help for
        command: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Upload(args) => commands::upload::upload_file_cmd(args),
        Commands::Download(args) => commands::download::download_file_cmd(args),
        Commands::Info(args) => commands::info::info_file_cmd(args),
        Commands::Help { command } => {
            let mut cmd = Cli::command();
            if let Some(sub) = command {
                if let Some(sc) = cmd.find_subcommand_mut(&sub) {
                    sc.print_help().unwrap();
                } else {
                    eprintln!("Unknown command '{}', showing general help:", sub);
                    cmd.print_help().unwrap();
                }
            } else {
                cmd.print_help().unwrap();
            }
            println!();
        }
    }
}
