use clap::Args;
use ffsend_api:: {
    action::info:: { Info, InfoResponse}, 
    client::ClientConfigBuilder, 
    file::remote_file::RemoteFile
};

use crate::commands::upload:: {
    read_tokens_from_file, OwnerToken
};

use url::Url;

#[derive(Args)]
pub struct InfoArgs {
    pub url: String,
}

pub fn info_cmd(args: InfoArgs) {
    match fetch_info(args.url) {
        Ok(info) => {
            println!("Download counts: {}", info.download_count());
            println!("Download expiry: {}", info.download_limit());
            println!("Download left: {}", info.download_left());
            println!("Time to live (ms) : {}", info.ttl_millis());
        }
        Err(e) => {
            eprintln!("Failed to fetch the file info: {}", e);
        }
    }
}

fn fetch_info(url_str: String) -> Result<InfoResponse, Box<dyn std::error::Error>> {
    let url = Url::parse(&url_str)?;
    let client = ClientConfigBuilder::default().build()?.client(true);

    let tokens = read_tokens_from_file("owner_token.json").unwrap_or_default();

    let mut remote_file = RemoteFile::parse_url(url.clone(), None)?;
    if let Some(OwnerToken { owner_token }) = tokens.get(remote_file.id()) {
        remote_file.set_owner_token(Some(owner_token.clone()));
    }

    let info = Info::new(&remote_file, None).invoke(&client)?;
    Ok(info)
}