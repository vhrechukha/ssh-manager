use crate::domain::delete_identity::enums;
use crate::domain::delete_identity::delete_identity::execute;

use crate::repositories::traits::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    match execute(repo) {
        Ok(()) => println!("This Config Identity has been deleted"),
        Err(enums::DeleteIdentityError::BadRequest) => println!("The request is invalid"),
        Err(enums::DeleteIdentityError::NotFound) => println!("Config Identities do not exist"),
        Err(enums::DeleteIdentityError::Unknown) => println!("An unknown error occurred"),
    }
}