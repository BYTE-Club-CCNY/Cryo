use clap::Args;
use ffsend_api::{
    action::delete::{self, Delete},
    api::Version,
    client::ClientConfigBuilder,
};
use url::Url;

#[derive(Args)]
pub struct DeleteArgs {
    /// URL to delete
    pub url: String,
}

pub fn delete_cmd(args: DeleteArgs) {
    let url = match Url::parse(&args.url) {
        Ok(u) => u,
        Err(_) => {
            println!("false");
            return;
        }
    };

    match delete_file(url) {
        Ok(true) => println!("true"),
        Ok(false) => println!("false"),
        Err(_) => println!("false"),
    }
}

fn delete_file(url: Url) -> Result<bool, Box<dyn std::error::Error>> {
    let client_config = ClientConfigBuilder::default()
        .build()?
        .client(true);

    let delete = Delete::new(Version::V3, url);
    let result = delete::invoke(delete, &client_config)?;

    Ok(result.deleted)
}
