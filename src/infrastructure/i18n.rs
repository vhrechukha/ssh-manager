use std::collections::HashMap;
use std::fs::{read_dir,read_to_string};
use std::io;
use std::path::Path;
use serde_json;
use std::sync::Mutex;
use lazy_static::lazy_static;

use crate::domain::switch_language::enums::Languages;
use crate::repositories::config::FileRepository;

pub struct Translations {
    data: HashMap<String, HashMap<String, String>>,
    default_language: String,
}

impl Translations {
    fn new(default_language: &str) -> Result<Self, std::io::Error> {
        let mut data = HashMap::new();

        let available_languages = vec![Languages::En.as_str(), Languages::Ua.as_str()];

        // Load translations for each language
        for language in available_languages {
            let translations = Self::load_translations(language)?;
            data.insert(language.to_string(), translations);
        }

        Ok(Translations { data, default_language: default_language.to_string() })
    }

    fn flatten_translations(
        file_name: &str,
        translations: HashMap<String, HashMap<String, String>>,
    ) -> HashMap<String, String> {
        let mut flattened_translations = HashMap::new();
        let file_name = Path::new(file_name).file_stem().unwrap().to_string_lossy();
    
        for (key, values) in translations {
            for (sub_key, value) in values {
                let flattened_key = format!("{}:{}.{}", file_name, key, sub_key);
                flattened_translations.insert(flattened_key, value);
            }
        }
        flattened_translations
    }
    
    fn load_translations(language: &str) -> Result<HashMap<String, String>, io::Error> {
        let folder_path = format!("locales/{}", language);
        let entries = read_dir(folder_path)?;
    
        let translations = entries.filter_map(|entry| {
            let entry = entry.ok()?;
            let file_path = entry.path();
            let file_name = file_path.file_name()?.to_string_lossy().into_owned();
            if file_name.ends_with(".json") {
                let contents = read_to_string(file_path).ok()?;
                let json_translations: HashMap<String, HashMap<String, String>> =
                    serde_json::from_str(&contents).ok()?;
                
                let flattened_translations = Self::flatten_translations(&file_name, json_translations);
                
                Some(flattened_translations)
            } else {
                None
            }
        }).fold(HashMap::new(), |mut acc, map| {
            acc.extend(map);
            acc
        });
    
        Ok(translations)
    }

    fn translate<'a>(&'a self, key: &'a str) -> Option<&'a str> {
        let language = &self.default_language;

        if let Some(language_translations) = self.data.get(language) {
            if let Some(message) = language_translations.get(key) {
                return Some(message);
            }
        }
        None
    }

    pub fn set_default_language(&mut self, default_language: &str) {
        self.default_language = default_language.to_string();
    }
}

// Global static variable to hold the translations
lazy_static! {
    static ref TRANSLATIONS: Mutex<Translations> = {
        let file_repository = FileRepository::new();
        let (default_language, _) = file_repository.read_data().unwrap();
        Mutex::new(Translations::new(&default_language).unwrap())
    };
}

pub fn set_default_language(default_language: &str) {
    let mut translations = TRANSLATIONS.lock().unwrap();
    translations.set_default_language(default_language);
}

// Function for translating a key
pub fn translate(key: &str) -> String {
    let translations = TRANSLATIONS.lock().unwrap();
    translations
        .translate(key)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("Translation not found for key: {}", key))
}