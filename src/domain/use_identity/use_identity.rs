use std::path::{Path};
use std::{convert::TryFrom, process::Stdio};
use std::sync::Arc;

use crate::domain;
use crate::domain::entities::{ConfigIdentity, Alias};
use crate::repositories::enums::{FindIdentityRepositoryError, FindIdentitiesRepositoryError};
use crate::repositories::traits::Repository;

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use execute::Execute;

use super::enums::UseIdentityError;
use super::structs::UseIdentityResponse;

pub fn execute(repo: Arc<dyn Repository>) -> Result<UseIdentityResponse, UseIdentityError> {
    let identities = match repo.find_all() {
        Ok(identities) => Ok(identities
            .into_iter()
            .map(|p| UseIdentityResponse {
                alias: String::from(p.alias),
                config_path: p.config_path.into(),
                hostname: String::from(p.hostname),
            })
            .collect::<Vec<UseIdentityResponse>>()),
        Err(FindIdentitiesRepositoryError::Unknown) => Err(UseIdentityError::Unknown),
    };

    match identities {
        Ok(val) => {
            let selections: Vec<String> = val.into_iter().map(|x| x.alias.to_string()).collect();

            if selections.is_empty() {
                return Err(UseIdentityError::NotFound);
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your Config Identity")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            println!("Your choose this identity: {}", selections[selection]);

            let identity_alias = &selections[selection];

            let config_identity: Result<UseIdentityResponse, UseIdentityError> = match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.find_one(alias) {
                   Ok(ConfigIdentity {
                       hostname,
                       config_path,
                       alias,
                   }) => Ok(UseIdentityResponse {
                           alias: String::from(alias),
                           config_path: String::from(config_path),
                           hostname: String::from(hostname),
                       }),
                   Err(FindIdentityRepositoryError::NotFound) => Err(UseIdentityError::NotFound),
            }
                Err(_) => todo!(), };

            let unwrapped_config_identity = config_identity.unwrap();

            // REFACTOR
            let identities_with_the_same_host = match repo.find_all_with_hostname(
                domain::entities::HostName(unwrapped_config_identity.clone().hostname)
            ) {
                Ok(identities) => Ok(identities
                    .into_iter()
                    .map(|p| UseIdentityResponse {
                        alias: String::from(p.alias),
                        config_path: p.config_path.into(),
                        hostname: String::from(p.hostname),
                    })
                    .collect::<Vec<UseIdentityResponse>>()),
                Err(FindIdentitiesRepositoryError::Unknown) => Err(UseIdentityError::Unknown),
            };
            let identities_with_the_same_host_unwrapped = identities_with_the_same_host.unwrap();
            let identities_with_the_same_host_length = identities_with_the_same_host_unwrapped.len();
            if identities_with_the_same_host_length >= 1 {
                for n in 0..identities_with_the_same_host_length {
                    let config_path_identity = String::from(identities_with_the_same_host_unwrapped[n].clone().config_path);
                    let path = Path::new(&dirs::home_dir().unwrap()).join(&config_path_identity);
        
                    let mut command = execute::command_args!("ssh-add", "-d", path);
                
                    command.stdout(Stdio::piped());
                    
                    command.execute_output().unwrap();
                }
            }

            let config_path_identity = String::from(unwrapped_config_identity.clone().config_path);
            let path = Path::new(&dirs::home_dir().unwrap()).join(&config_path_identity);

            let mut command = execute::command_args!("ssh-add", path);
        
            command.stdout(Stdio::piped());
            
            command.execute_output().unwrap();
        
           return Ok(unwrapped_config_identity);
        },
        Err(_err) => {
            println!("UseIdentityError: {:#?}", _err);

            panic!()
        }
    }
}