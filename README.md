# Cryo
A secure file sharing cli written in Rust.



Update Logs:
April 17th - Added a new upload subcommand using clap to handle file uploads via the ffsend-api crate.
The actual logic for the upload is located in a separate commands/upload.rs module to keep the codebase clean and modular.
In main.rs, we parse the CLI input and send to the correct subcommand (Upload) by importing the UploadArgs and calling upload_file_cmd.
use command to upload
cargo run -- upload <your_path>

April 23rd - Added the download command. 
    1.  Parse the input URL into a RemoteFile descriptor using RemoteFile::parse_url.
	2.	Fetch metadata (via the Metadata action) to retrieve the original filename from the ffsend service.
	3.	Construct the local target path using the fetched filename instead of a placeholder.
	4.	Pass the metadata response into the Download action to avoid redundant metadata calls.
	5.	Clean up imports to include the new metadata action types.
Command: cargo run -- download <URL>