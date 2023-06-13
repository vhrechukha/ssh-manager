use std::sync::Arc;

use crate::domain::switch_language::enums::Languages;
use crate::infrastructure::i18n::set_default_language;
use crate::repositories::traits::Repository;
use crate::{infrastructure::i18n::translate};

use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

pub fn execute(repo: Arc<dyn Repository>) -> Result<(), Box<dyn std::error::Error>> {
    let selections: [&str; 2] = ["english", "українська"];

    let selection = Select::with_theme(&ColorfulTheme::default())
    .with_prompt(translate("switch_language:domain.chooseLanguage"))
    .default(0)
    .items(&selections)
    .interact()
    .unwrap();

    let language_selection = selections[selection];
    let language_code = match language_selection {
        "english" => Languages::En.as_str(),
        "українська" => Languages::Ua.as_str(),
        _ => panic!("{}", translate("switch_language:domain.unknownLanguage")),
    };

    repo.write_language(language_code)?;
    set_default_language(language_code);

    return Ok(());
}