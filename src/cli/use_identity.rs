use crate::repositories::traits::Repository;
use crate::domain::{use_identity, self};
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) -> Result<(), domain::use_identity::Error> {
    if let Err(err) = use_identity::execute(repo.clone()) {
        match err {
            use_identity::Error::BadRequest | use_identity::Error::NotFound | use_identity::Error::Unknown => {
                return Err(err.into()); // Propagate the error
            }
        }
    }
    
    println!("Added using of Config Identity with such alias");
    Ok(())
}

