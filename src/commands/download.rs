use clap::Args;
use ffsend_api::{
    action::download::Download,
    api::Version,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use ffsend_api::action::metadata::{Metadata as MetadataAction, MetadataResponse};
use url::Url;
use std::path::PathBuf;

#[derive(Args)]
pub struct DownloadArgs {
    pub url: String,
}

/// download subcommand logiic
pub fn download_file_cmd(args: DownloadArgs) {
    match download_file(args.url) {
        Ok(file_path) => {
            println!("Downloaded to: {}", file_path.display());
        }
        Err(e) => {
            eprintln!("Download failed: {}", e);
        }
    }
}

fn download_file(url: String) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let client_config = ClientConfigBuilder::default().build()?;
    let client = client_config.client(true);
    let version = Version::V3;

    let url = Url::parse(&url)?;
    let file = RemoteFile::parse_url(url.clone(), None)?;

    // fetch metadata to get the original file name
    let metadata_response: MetadataResponse = MetadataAction::new(&file, None, true)
        .invoke(&client)?;
    let file_name = metadata_response.metadata().name();
    let target_path: PathBuf = std::env::current_dir()?.join(file_name);

    // build download action with the fetched metadata to avoid refetching
    let download = Download::new(
        version,
        &file,
        target_path.clone(),
        None, 
        true,
        Some(metadata_response),
    );

    download.invoke(&client, None)?;
    Ok(target_path)
}