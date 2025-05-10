use clap::Args;
use ffsend_api::{
    action::{
        info::{Info, InfoResponse},
        metadata::{Metadata, MetadataResponse},
    },
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use crate::commands::upload::{read_tokens_from_file, OwnerToken};
use url::Url;

#[derive(Args)]
pub struct InfoArgs {
    pub url: String,
}

pub fn info_cmd(args: InfoArgs) {
    let url = Url::parse(&args.url).expect("Invalid URL");
    let client = ClientConfigBuilder::default()
        .build()
        .expect("Failed to build client config")
        .client(true);

    let tokens = read_tokens_from_file("owner_token.json").unwrap_or_default();

    let mut file = RemoteFile::parse_url(url.clone(), None)
        .expect("Failed to parse URL into RemoteFile");

    let meta: MetadataResponse = Metadata::new(&file, None, true)
        .invoke(&client)
        .expect("Failed to fetch public metadata");

    if let Some(OwnerToken { owner_token }) = tokens.get(file.id()) {
        file.set_owner_token(Some(owner_token.clone()));
        let info: InfoResponse = Info::new(&file, None)
            .invoke(&client)
            .expect("Failed to fetch management info");

        println!("-- File Info --");
        println!("Download count : {}", info.download_count());
        println!("Download limit : {}", info.download_limit());
        println!("Download left  : {}", info.download_left());
    }

    println!("-- File Metadata --");
    println!("File name      : {}", meta.metadata().name());
    println!("Content length : {}", meta.size());
}