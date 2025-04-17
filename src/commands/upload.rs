use clap::Args;
use std::path::PathBuf;
use ffsend_api::{
    action::upload::Error,
    action::upload::Upload,
    api::Version,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use url::Url;

#[derive(Args)]
pub struct UploadArgs {
    /// Path to the file to upload
    pub file: String,
}

pub fn upload_file_cmd(args: UploadArgs) {
    let path = PathBuf::from(args.file);

    match upload_file(path) {
        Ok(remote_file) => {
            let share_url = remote_file.download_url(true);
            println!("Share URL: {}", share_url);
        }
        Err(err) => {
            eprintln!("Failed to upload file: {}", err);
        }
    }
}

fn upload_file(path: PathBuf) -> Result<RemoteFile, Error> {
    let client_config = ClientConfigBuilder::default()
        .build()
        .expect("Failed to build client config");
    let client = client_config.client(true);

    let version = Version::V3;

    let upload = Upload::new(
        version,
        Url::parse("https://send.vis.ee/").expect("Invalid URL"),
        path,
        None, // password: optional
        None, // expiry: optional
        None, // download limit: optional
    );

    Upload::invoke(upload, &client, None)
}
