use dialoguer::Input;

use crate::infrastructure::i18n::translate;

pub fn prompt_alias() -> Result<String, ()> {
    match Input::new().with_prompt(translate("prompts:inputs.alias")).interact_text() {
        Ok(alias) => Ok(alias),
        _ => Err(()),
    }
}

pub fn prompt_hostname() -> Result<String, ()> {
    match Input::new().with_prompt(translate("prompts:inputs.hostname")).interact_text() {
        Ok(hostname) => Ok(hostname),
        _ => Err(()),
    }
}

pub fn prompt_path() -> Result<String, ()> {
    match Input::new().with_prompt(translate("prompts:inputs.globalPath")).interact_text() {
        Ok(path) => Ok(path),
        _ => Err(()),
    }
}
