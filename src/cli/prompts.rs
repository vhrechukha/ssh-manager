use dialoguer::Input;

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
