use crate::cli::prompts::{prompt_alias, prompt_hostname, prompt_path};

use crate::domain::create_identity::{structs,enums};
use crate::domain::create_identity::create_identity::execute;

use crate::repositories::traits::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    let alias = prompt_alias();
    let hostname = prompt_hostname();
    let config_path = prompt_path();

    let req = match (alias, hostname, config_path) {
        (Ok(alias), Ok(hostname), Ok(config_path)) => structs::CreateIdentityRequest {
            alias,
            hostname,
            config_path,
        },
        _ => {
            println!("An error occurred during the prompt");
            return;
        }
    };
    match execute(repo, req) {
        Ok(res) => println!("Added Config Identity with such alias: {:?}", res.alias),
        Err(enums::CreateIdentityError::BadRequest) => println!("The request is invalid"),
        Err(enums::CreateIdentityError::Conflict) => println!("This Config Identity already exists"),
        Err(enums::CreateIdentityError::Unknown) => println!("An unknown error occurred"),
    };
}
