mod add_identity;
mod use_identity;
mod delete_identity;
mod switch_language;
mod prompts;

use crate::{repositories::traits::Repository, infrastructure::i18n::translate};
use crate::domain;
use dialoguer::{theme::ColorfulTheme, Select};
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    while let Ok(index) = Select::with_theme(&ColorfulTheme::default())
        .with_prompt(translate("prompts:menu.title"))
        .items(&[
            translate("prompts:menu.useIdentity"), 
            translate("prompts:menu.addIdentity"), 
            translate("prompts:menu.deleteIdentity"), 
            translate("prompts:menu.switchLanguage"), 
            translate("prompts:menu.exit"), 
        ])
        .default(0)
        .interact()
    {
        match index {
            0 => {
                if let Err(err) = use_identity::run(repo.clone()) {
                    match err {
                        domain::use_identity::enums::UseIdentityError::NotFound => {
                            println!("{}", translate("general:errors.noConfigIdentitiesFound"));
                        }
                        domain::use_identity::enums::UseIdentityError::Unknown => {
                            println!("{}", translate("general:errors.unknownError"));
                        }
                    }
                    continue; // Restart the loop and prompt for index again
                }
            }
            1 => add_identity::run(repo.clone()),
            2 => delete_identity::run(repo.clone()),
            3 => {
                if let Err(_) = switch_language::run(repo.clone()) {
                    println!("{}", translate("general:errors.unknownError"));
                    continue; // Restart the loop and prompt for index again
                }
            }
            4 => break,
            _ => continue,
        };
    }
}