# Cryo
A secure file sharing cli written in Rust.


I fixed and completed the upload command.

Added a new upload subcommand using clap to handle file uploads via the ffsend-api crate.
The actual logic for the upload is located in a separate commands/upload.rs module to keep the codebase clean and modular.
In main.rs, we parse the CLI input and send to the correct subcommand (Upload) by importing the UploadArgs and calling upload_file_cmd.
use command to upload
cargo run -- upload <your_path>