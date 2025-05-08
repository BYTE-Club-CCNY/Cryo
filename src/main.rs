mod commands;
use std::io::{self, Write};
use clap::{Parser, Subcommand};
use terminal_menu:: {
    menu, label, button, run, mut_menu
};


#[derive(Parser)]
#[clap(name = "Cryo", version = "0.1", author = "al", about = "A CLI for file sharing using ffsend")]
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
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 1 {
        run_menu();
    // Goes back to using regular Clap parsing
    } else {
        let cli = Cli::parse();

        match cli.command {
            Commands::Upload(args) => commands::upload::upload_file_cmd(args),
            Commands::Download(args) => commands::download::download_file_cmd(args),
            Commands::Exists(args) => commands::exists::exists_cmd(args),
            Commands::Delete(args) => commands::delete::delete_cmd(args),
        }
    }
}

fn run_menu() {
    loop {
        let main_menu = menu(vec![
            label("----------------------------"),
            label("   Cryo, Secure File Sharer  "),
            label("----------------------------"),    
            button("Upload"),
            button("Download"),
            button("Exists"),
            button("Delete"),
            button("Exit"),
            ]);

        run(&main_menu);
        
        // don't make it mut, as menu alr mutable
        // guard alive thru the match
        let guard = mut_menu(&main_menu);
        match guard.selected_item_name() {
            "Upload" => {
                print!("Enter file path to upload: ");
                io::stdout().flush().unwrap();
                let mut file = String::new();
                io::stdin().read_line(&mut file).unwrap();
                commands::upload::upload_file_cmd(
                    commands::upload::UploadArgs { file: file.trim().into() }
                );
            }

            "Download" => {
                print!("Enter the download link: ");
                io::stdout().flush().unwrap();
                let mut url = String::new();
                io::stdin().read_line(&mut url).unwrap();
                commands::download::download_file_cmd(
                    commands::download::DownloadArgs { url: url.trim().into() }
                );
            }

            "Exists" => {
                print!("Enter link to check if file exists: ");
                io::stdout().flush().unwrap();
                let mut url = String::new();
                io::stdin().read_line(&mut url).unwrap();
                commands::exists::exists_cmd(
                    commands::exists::ExistsArgs { url: url.trim().into() }
                );
            }

            "Delete" => {
                print!("Enter link to delete: ");
                io::stdout().flush().unwrap();
                let mut url = String::new();
                io::stdin().read_line(&mut url).unwrap();
                commands::delete::delete_cmd(
                    commands::delete::DeleteArgs { url: url.trim().into() }
                );
            }

            "Exit" => {
                println!("Goodbye!");
                break;
            }

            _ => {
                eprintln!("Invalid Selection, try again!");
            }
        }

        // pause before showing menu again
        println!("\nPress Enter to continue...");
        let _ = io::stdin().read_line(&mut String::new());
    }
}