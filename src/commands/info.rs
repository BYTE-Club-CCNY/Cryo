use clap::Args;
use ffsend_api::{
    action::metadata::{Metadata as MetadataAction, MetadataResponse},
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use url::Url;

#[derive(Args)]
/// Show metadata for a shared file URL
pub struct InfoArgs {
    /// URL of the file to show info for
    pub url: String,
}

/// Execute the info subcommand: fetch and display metadata
pub fn info_file_cmd(args: InfoArgs) {
    match info_file(args.url) {
        Ok(resp) => println!("{:#?}", resp.metadata()),
        Err(e) => eprintln!("Info failed: {}", e),
    }
}

/// Internal helper to perform the metadata request
fn info_file(url: String) -> Result<MetadataResponse, Box<dyn std::error::Error>> {
    // build HTTP client
    let client = ClientConfigBuilder::default().build()?.client(true);

    // parse URL and create RemoteFile object
    let url = Url::parse(&url)?;
    let file = RemoteFile::parse_url(url, None)?;

    // invoke metadata action without version key
    let resp = MetadataAction::new(&file, None, true).invoke(&client)?;
    Ok(resp)
}
