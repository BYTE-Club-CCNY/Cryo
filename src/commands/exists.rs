use clap::Args;
use ffsend_api::{
    action::exists::Exists,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use url::Url;

/// Arguments for the `exists` subcommand
#[derive(Args)]
pub struct ExistsArgs {
    /// URL to check
    pub url: String,
}

/// exists subcommand logic
pub fn exists_cmd(args: ExistsArgs) {
    match check_exists(args.url) {
        Ok(exists) => println!("{}", exists),
        Err(_) => println!("This file was deleted or does not exist."),
    }
}

fn check_exists(url: String) -> Result<bool, Box<dyn std::error::Error>> {
    let url = Url::parse(&url)?;

    let client_config = ClientConfigBuilder::default().build()?;
    let client = client_config.client(true);

    let remote_file = RemoteFile::parse_url(url, None)?;
    let exists_action = Exists::new(&remote_file);
    let result = Exists::invoke(exists_action, &client)?;

    Ok(result.exists())
}
