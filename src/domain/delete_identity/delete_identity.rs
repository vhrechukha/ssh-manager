use crate::domain::entities::Alias;
use crate::repositories::enums::{DeleteIdentityRepositoryError, FindIdentitiesRepositoryError};
use crate::repositories::traits::Repository;
use std::convert::TryFrom;
use std::sync::Arc;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

use super::enums::DeleteIdentityError;
use super::structs::DeleteIdentityResponse;
use crate::{infrastructure::i18n::translate};

pub fn execute(repo: Arc<dyn Repository>) -> Result<(), DeleteIdentityError> {
    let identities = match repo.find_all() {
        Ok(identities) => Ok(identities
            .into_iter()
            .map(|p| DeleteIdentityResponse {
                alias: String::from(p.alias),
                config_path: p.config_path.into(),
                hostname: String::from(p.hostname),
            })
            .collect::<Vec<DeleteIdentityResponse>>()),
        Err(FindIdentitiesRepositoryError::Unknown) => Err(DeleteIdentityError::Unknown),
    };

    match identities {
        Ok(val) => {
            let selections: Vec<String> = val.into_iter().map(|x| x.alias.to_string()).collect();

            if selections.is_empty() {
                return Err(DeleteIdentityError::NotFound);
            }

            // Move it to cli
            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt(translate("delete_identity:domain.chooseConfig"))
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            let identity_alias = &selections[selection];

           match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.delete(alias) {
                    Ok(()) => Ok(()),
                    Err(DeleteIdentityRepositoryError::NotFound) => Err(DeleteIdentityError::NotFound),
                    Err(DeleteIdentityRepositoryError::Unknown) => Err(DeleteIdentityError::Unknown),
                },
                _ => Err(DeleteIdentityError::BadRequest),
            }
        },
        Err(_err) => {
            panic!()
        }
    }
}