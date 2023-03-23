use crate::repositories::config::Repository;
use crate::domain::use_identity;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    match use_identity::execute(repo) {
        Ok(res) => println!("Added using of Config Identity with such alias: {:?}", res.alias),
        Err(use_identity::Error::BadRequest) => println!("The request is invalid"),
        Err(use_identity::Error::NotFound) => println!("The Config Identity does not exist"),
        Err(use_identity::Error::Unknown) => println!("An unknown error occurred"),
    }
}
