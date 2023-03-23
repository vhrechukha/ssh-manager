use std::path::{Path};
use std::{convert::TryFrom, process::Stdio};
use std::sync::Arc;

use crate::repositories::config::{Repository, FindIdentityError, FindIdentitiesError};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;
use execute::Execute;

use super::entities::{ConfigIdentity, Alias};

pub struct Response {
    pub alias: String,
    pub hostname: String,
    pub config_path: String,
}

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
            let selections: &Vec<String> = &val.into_iter().map(|x| x.alias.to_string()).collect();

            let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Pick your flavor")
            .default(0)
            .items(&selections[..])
            .interact()
            .unwrap();
    
            println!("Your choose this identity: {}", selections[selection]);

            let identity_alias = &selections[selection];

            let config_identity = match Alias::try_from(identity_alias.to_owned()) {
                Ok(alias) => match repo.find_one(alias) {
                   Ok(ConfigIdentity {
                       hostname,
                       config_path,
                       alias,
                   }) => {
                       let config_path_identity = String::from(config_path);
                       let path = Path::new(&dirs::home_dir().unwrap()).join(&config_path_identity);
       
                       let mut command = execute::command_args!("ssh-add", path);
                   
                       command.stdout(Stdio::piped());
                       
                       let output = command.execute_output().unwrap();
                   
                       println!("Output: {}", String::from_utf8(output.stdout).unwrap());
       
                       return {Ok(Response {
                           alias: String::from(alias),
                           config_path: config_path_identity,
                           hostname: String::from(hostname),
                       })
                   }
               },
                   Err(FindIdentityError::NotFound) => Err(Error::NotFound),
                   Err(FindIdentityError::Unknown) => Err(Error::Unknown),
               },
               _ => Err(Error::BadRequest),
           };

           return config_identity;
        },
        Err(_err) => {
            panic!()
        }
    }
}