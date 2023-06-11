use crate::cli::prompts::{prompt_alias, prompt_hostname, prompt_path};
use crate::domain::create_identity;
use crate::repositories::traits::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    let alias = prompt_alias();
    let hostname = prompt_hostname();
    let config_path = prompt_path();

    let req = match (alias, hostname, config_path) {
        (Ok(alias), Ok(hostname), Ok(config_path)) => create_identity::Request {
            alias,
            hostname,
            config_path,
        },
        _ => {
            println!("An error occurred during the prompt");
            return;
        }
    };
    match create_identity::execute(repo, req) {
        Ok(res) => println!("Added Config Identity with such alias: {:?}", res.alias),
        Err(create_identity::Error::BadRequest) => println!("The request is invalid"),
        Err(create_identity::Error::Conflict) => println!("This Config Identity already exists"),
        Err(create_identity::Error::Unknown) => println!("An unknown error occurred"),
    };
}
