mod add_identity;
mod use_identity;
mod delete_identity;

use crate::{repositories::config::Repository, domain};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    while let Ok(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Make your choice")
        .items(&["Use Identity", "Add Identity", "Delete Identity", "Exit"])
        .default(0)
        .interact()
    {
        match index {
            0 => {
                if let Err(err) = use_identity::run(repo.clone()) {
                    match err {
                        domain::use_identity::Error::BadRequest => {
                            println!("The request is invalid");
                        }
                        domain::use_identity::Error::NotFound => {
                            println!("The Config Identities does not exist");
                        }
                        domain::use_identity::Error::Unknown => {
                            println!("An unknown error occurred");
                        }
                        _ => {
                            println!("Error: {:?}", err);
                        }
                    }
                    continue; // Restart the loop and prompt for index again
                }
            }
            1 => add_identity::run(repo.clone()),
            2 => delete_identity::run(repo.clone()),
            3 => break,
            _ => continue,
        };
    }
}

pub fn prompt_alias() -> Result<String, ()> {
    match Input::new().with_prompt("Alias for your identity").interact_text() {
        Ok(alias) => Ok(alias),
        _ => Err(()),
    }
}

pub fn prompt_hostname() -> Result<String, ()> {
    match Input::new().with_prompt("Hostname").interact_text() {
        Ok(hostname) => Ok(hostname),
        _ => Err(()),
    }
}

pub fn prompt_path() -> Result<String, ()> {
    match Input::new().with_prompt("Global path").interact_text() {
        Ok(path) => Ok(path),
        _ => Err(()),
    }
}
