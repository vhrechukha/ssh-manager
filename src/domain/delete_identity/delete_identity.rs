use crate::domain::entities::Alias;
use crate::repositories::enums::{DeleteIdentityError, FindIdentitiesError};
use crate::repositories::traits::Repository;
use std::convert::TryFrom;
use std::sync::Arc;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use super::enums::UseIdentityError;
use super::structs::DeleteIdentityResponse;

pub fn execute(repo: Arc<dyn Repository>) -> Result<(), UseIdentityError> {
    let identities = match repo.find_all() {
        Ok(identities) => Ok(identities
            .into_iter()
            .map(|p| DeleteIdentityResponse {
                alias: String::from(p.alias),
                config_path: p.config_path.into(),
                hostname: String::from(p.hostname),
            })
            .collect::<Vec<DeleteIdentityResponse>>()),
        Err(FindIdentitiesError::Unknown) => Err(UseIdentityError::Unknown),
    };

    match identities {
        Ok(val) => {
            let selections: &Vec<String> = &val.into_iter().map(|x| x.alias.to_string()).collect();

            // Move it to cli
            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your Config Identity")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            let identity_alias = &selections[selection];

           match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.delete(alias) {
                    Ok(()) => Ok(()),
                    Err(DeleteIdentityError::NotFound) => Err(UseIdentityError::NotFound),
                    Err(DeleteIdentityError::Unknown) => Err(UseIdentityError::Unknown),
                },
                _ => Err(UseIdentityError::BadRequest),
            }
        },
        Err(_err) => {
            panic!()
        }
    }
}