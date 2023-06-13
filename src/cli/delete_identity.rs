use crate::domain::delete_identity::enums;
use crate::domain::delete_identity::delete_identity::execute;

use crate::infrastructure::i18n::translate;
use crate::repositories::traits::Repository;
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    match execute(repo) {
        Ok(()) => println!("{}", translate("delete_identity:cli.successful")),
        Err(enums::DeleteIdentityError::BadRequest) => println!("{}", translate("general:errors.requestInvalidError")),
        Err(enums::DeleteIdentityError::NotFound) => println!("{}", translate("general:errors.noConfigIdentitiesFound")),
        Err(enums::DeleteIdentityError::Unknown) => println!("{}", translate("general:errors.unknownError")),
    }
}