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
use std::path::PathBuf;
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

    // expiry time is in seconds, 604800 = 7 days

    // The following code is what is used in ffsend itself for it's upload command.
    // Seems like best practice, hence why I changed it
    let params = {
        let params = ParamsDataBuilder::default()
            .download_limit(Some(5))
            // TODO: Debug why expiry time is causing issues
            .expiry_time(Some(259_800))
            .build()
            .unwrap();

        if params.is_empty() {
            None
        } else {
            Some(params)
        }
    };

    // let progress_bar = Arc::new(Mutex::new(ProgressBar::new_upload()));
    // let progress_reporter: Arc<Mutex<dyn ProgressReporter>> = progress_bar;
    
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
