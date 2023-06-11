use crate::repositories::traits::Repository;

use crate::domain::use_identity::{enums};
use crate::domain::use_identity::use_identity::execute;

use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) -> Result<(), enums::UseIdentityError> {
    if let Err(err) = execute(repo.clone()) {
        match err {
            enums::UseIdentityError::BadRequest | enums::UseIdentityError::NotFound | enums::UseIdentityError::Unknown => {
                return Err(err.into()); // Propagate the error
            }
        }
    }
    
    println!("Added using of Config Identity with such alias");
    Ok(())
}

