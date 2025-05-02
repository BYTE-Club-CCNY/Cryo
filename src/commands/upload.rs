use clap::Args;
use ffsend_api::{
    action::{
        params::ParamsDataBuilder,
        upload::{Error, Upload},
    },
    api::Version,
    client::ClientConfigBuilder,
    file::remote_file::RemoteFile,
};
use serde::{Deserialize, Serialize};

use std::{
    collections::HashMap,
    fs::{File, OpenOptions},
    io::{BufReader, Write},
    path::PathBuf,
};
use url::Url;


#[derive(Args)]
pub struct UploadArgs {
    /// Path to the file to upload
    pub file: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct OwnerToken {
    owner_token: String,
}

pub fn upload_file_cmd(args: UploadArgs) {
    let path = PathBuf::from(args.file);

    
    match upload_file(path) {
        Ok(remote_file) => {
            let share_url: Url = remote_file.download_url(true);
            println!("Share URL: {}", share_url);
            if let Some(token) = remote_file.owner_token() {
                let file_id = remote_file.id();
                if let Err(e) = save_token(file_id, token) {
                    eprintln!("failed to save owner token: {}", e);
                } else {
                    println!("onwer token saved");
                }
            } else {
                println!("no Onwer token found from server");
            }
        }
        // Added this line to see exactly what kind of error we get when uploading.
        Err(Error::Upload(e)) => {
            eprintln!("Failed to upload file: {}", e);
        }
        Err(e) => {
            eprintln!("Some other error occurred: {:?}", e);
        }
    }
}

fn upload_file(path: PathBuf) -> Result<RemoteFile, Error> {
    let client_config = ClientConfigBuilder::default()
        .build()
        .expect("Failed to build client config");

    let client = client_config.client(true);

    let version = Version::V3;

    // The following code is what is used in ffsend itself for it's upload command.
    // Seems like best practice, hence why I changed it
    let params = {
        let params = ParamsDataBuilder::default()
            .download_limit(Some(5))
            // TODO: Debug why expiry time is causing issues
            .expiry_time(Some(3600))
            .build()
            .unwrap();

        if params.is_empty() {
            None
        } else {
            Some(params)
        }
    };

    
    let upload = Upload::new(
        version,
        Url::parse("https://send.vis.ee/").expect("Invalid URL"),
        path,
        None,
        None,
        params,
    );

    Upload::invoke(upload, &client, None)
}

fn save_token(file_id: &str, owner_token: &str) -> std::io::Result<()> {
    let path_file = "owner_token.json";

    let mut tokens: HashMap<String, OwnerToken> = {
        // open file and deseriilize
        match File::open(path_file) {
            Ok(file) => {
                let reader = BufReader::new(file);
                serde_json::from_reader(reader).unwrap_or_default() 
            }
            // else no file yet :(
            Err(_) => HashMap::new(), 
        }
    };
    // writes out the object, file id as key and token as value
    tokens.insert(
        file_id.to_string(),
        OwnerToken {
            owner_token: owner_token.to_string(),
        },
    );

    let json = serde_json::to_string_pretty(&tokens)?;
    let mut file = OpenOptions::new().create(true).write(true).truncate(true).open(path_file)?;
    file.write_all(json.as_bytes())?;

    Ok(())
}
