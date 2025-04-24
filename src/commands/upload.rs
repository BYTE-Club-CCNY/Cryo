use clap::Args;
use std::path::PathBuf;
use ffsend_api::{
    action::upload::Error,
    action::upload::Upload,
    api::Version,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
    action::params::ParamsData,
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

    // expiry time is in seconds, 605800 = 7 days
    let params = ParamsData::from(Some(5), Some(604800)); 

    let upload = Upload::new(
        version,
        Url::parse("https://send.vis.ee/").expect("Invalid URL"),
        path,
        None, 
        None, 
        Some(params), 
    );

    Upload::invoke(upload, &client, None)
}
