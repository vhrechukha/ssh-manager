use crate::cli::prompts::{prompt_alias, prompt_hostname, prompt_path};

use crate::domain::create_identity::{structs,enums};
use crate::domain::create_identity::create_identity::execute;

use crate::infrastructure::i18n::translate;
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
            println!("{}", translate("general:errors.promptError"));
            return;
        }
    };
    match execute(repo, req) {
        Ok(res) => println!("{} {:?}", translate("add_identity:cli.successful"), res.alias),
        Err(enums::CreateIdentityError::BadRequest) => println!("{}", translate("general:errors.requestInvalidError")),
        Err(enums::CreateIdentityError::Conflict) => println!("{}", translate("add_identity:cli.alreadyExistsError")),
        Err(enums::CreateIdentityError::Unknown) => println!("{}", translate("general:errors.unknownError")),
    };
}
