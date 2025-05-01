use clap::Args;
use ffsend_api::{
    action::exists::Exists,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use url::Url;

#[derive(Args)]
pub struct ExistsArgs {
    /// URL to check
    pub url: String,
}

pub fn exists_cmd(args: ExistsArgs) {
    let url = match Url::parse(&args.url) {
        Ok(u) => u,
        Err(_) => {
            println!("false");
            return;
        }
    };

    match check_exists(url) {
        Ok(result) => println!("{}", result),
        Err(_) => println!("false"),
    }
}

fn check_exists(url: Url) -> Result<bool, Box<dyn std::error::Error>> {
    let client_config = ClientConfigBuilder::default()
        .build()?
        .client(true);

    let remote_file = RemoteFile::parse_url(url, None)?;
    let exists = Exists::new(&remote_file);
    let result = Exists::invoke(exists, &client_config)?;

    Ok(result.exists())
}
