mod add_identity;
mod use_identity;
mod delete_identity;
mod prompts;

use crate::repositories::traits::Repository;
use crate::domain;
use dialoguer::{theme::ColorfulTheme, Select};
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
                        domain::use_identity::enums::UseIdentityError::BadRequest => {
                            println!("The request is invalid");
                        }
                        domain::use_identity::enums::UseIdentityError::NotFound => {
                            println!("The Config Identities does not exist");
                        }
                        domain::use_identity::enums::UseIdentityError::Unknown => {
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