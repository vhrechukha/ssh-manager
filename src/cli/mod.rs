mod add_identity;
mod use_identity;
mod delete_identity;

use crate::repositories::config::Repository;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use std::sync::Arc;

pub fn run(repo: Arc<dyn Repository>) {
    loop {
        let choices = [
            "Use Identity",
            "Add Identity",
            "Delete Identity",
            "Exit",
        ];
        let index = match Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Make your choice")
            .items(&choices)
            .default(0)
            .interact()
        {
            Ok(index) => index,
            _ => continue,
        };

        match index {
            0 => use_identity::run(repo.clone()),
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
