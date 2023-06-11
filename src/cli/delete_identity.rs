use crate::domain::delete_identity;
use crate::repositories::traits::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    match delete_identity::execute(repo) {
        Ok(()) => println!("This Config Identity has been deleted"),
        Err(delete_identity::Error::BadRequest) => println!("The request is invalid"),
        Err(delete_identity::Error::NotFound) => println!("This Config Identity does not exist"),
        Err(delete_identity::Error::Unknown) => println!("An unknown error occurred"),
    }
}