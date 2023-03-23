use crate::repositories::config::{Repository, FindIdentitiesError, DeleteError};
use std::convert::TryFrom;
use std::sync::Arc;
use super::entities::{Alias};
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub struct Response {
    pub alias: String,
    pub hostname: String,
    pub config_path: String,
}

pub enum Error {
    Unknown,
    NotFound,
    BadRequest,
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<(), Error> {
    let identities = match repo.find_all() {
        Ok(identities) => Ok(identities
            .into_iter()
            .map(|p| Response {
                alias: String::from(p.alias),
                config_path: p.config_path.into(),
                hostname: String::from(p.hostname),
            })
            .collect::<Vec<Response>>()),
        Err(FindIdentitiesError::Unknown) => Err(Error::Unknown),
    };

    match identities {
        Ok(val) => {
            let selections: &Vec<String> = &val.into_iter().map(|x| x.alias.to_string()).collect();

            // Move it to cli
            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            println!("You decided to delete this identity: {}", selections[selection]);

            let identity_alias = &selections[selection];

           match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.delete(alias) {
                    Ok(()) => Ok(()),
                    Err(DeleteError::NotFound) => Err(Error::NotFound),
                    Err(DeleteError::Unknown) => Err(Error::Unknown),
                },
                _ => Err(Error::BadRequest),
            }
        },
        Err(_err) => {
            panic!()
        }
    }
}