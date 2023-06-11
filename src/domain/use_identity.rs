use std::path::{Path};
use std::{convert::TryFrom, process::Stdio};
use std::sync::Arc;

use crate::domain;
use crate::repositories::config::{FindIdentityError, FindIdentitiesError};
use crate::repositories::traits::Repository;

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use execute::Execute;

use super::entities::{ConfigIdentity, Alias};

#[derive(Clone, Debug)]
pub struct Response {
    pub alias: String,
    pub hostname: String,
    pub config_path: String,
}

#[derive(Debug)]
pub enum Error {
    BadRequest,
    NotFound,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>) -> Result<Response, Error> {
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
            let selections: Vec<String> = val.into_iter().map(|x| x.alias.to_string()).collect();

            if selections.is_empty() {
                return Err(Error::NotFound);
            }

            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your Config Identity")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            println!("Your choose this identity: {}", selections[selection]);

            let identity_alias = &selections[selection];

            let config_identity: Result<Response, Error> = match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.find_one(alias) {
                   Ok(ConfigIdentity {
                       hostname,
                       config_path,
                       alias,
                   }) => Ok(Response {
                           alias: String::from(alias),
                           config_path: String::from(config_path),
                           hostname: String::from(hostname),
                       }),
                   Err(FindIdentityError::NotFound) => Err(Error::NotFound),
                   Err(FindIdentityError::Unknown) => Err(Error::Unknown),
            }
                Err(_) => todo!(), };

            let unwrapped_config_identity = config_identity.unwrap();

            // REFACTOR
            let identities_with_the_same_host = match repo.find_all_with_hostname(
                domain::entities::HostName(unwrapped_config_identity.clone().hostname)
            ) {
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
            println!("Error: {:#?}", _err);

            panic!()
        }
    }
}