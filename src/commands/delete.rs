use clap::Args;
use ffsend_api::{
    action::delete::Delete,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use crate::commands::upload::{read_tokens_from_file, OwnerToken};
use std::{collections::HashMap, error::Error};
use url::Url;

#[derive(Args)]
pub struct DeleteArgs {
    /// URL to delete
    pub url: String,
}

/// delete subcommand logic
pub fn delete_cmd(args: DeleteArgs) {
    match delete_file(args.url) {
        Ok(true) => println!("File deleted"),
        Ok(false) => eprintln!("Failed to delete file"),
        Err(e) => eprintln!("Delete failed: {}", e),
    }
}

fn delete_file(url: String) -> Result<bool, Box<dyn Error>> {
    let url = Url::parse(&url)?;

    let client_config = ClientConfigBuilder::default().build()?;
    let client = client_config.client(true);

    let file = RemoteFile::parse_url(url.clone(), None)?;
    let file_id = file.id();

    // Load tokens and fetch owner token for the file
    let tokens: HashMap<String, OwnerToken> = read_tokens_from_file("owner_token.json")?;
    let token = tokens
        .get(file_id)
        .ok_or("Token not found")?
        .owner_token
        .clone();

    let delete = Delete::new(&file, Some(token.into_bytes()));
    let result = delete.invoke(&client);

    Ok(result.is_ok())
}
