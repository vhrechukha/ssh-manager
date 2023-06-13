use std::fs;

use crate::domain::{entities::{ConfigIdentity, HostName, ConfigPath, Alias}, switch_language::enums::Languages};
use std::{path::PathBuf};

use crate::repositories::enums::{FindIdentityRepositoryError, FindIdentitiesRepositoryError, AddIdentityRepositoryError, DeleteIdentityRepositoryError};

use super::structs::ConfigData;

pub struct FileRepository {
    data_file_path: PathBuf,
}

impl FileRepository {
    pub fn new() -> Self {
        let home_dir = dirs::home_dir().expect("Failed to get home directory");
        let config_dir = home_dir.join(".config");
        let brew_package_dir = config_dir.join("ssh_manager");

        // Create the directory if it doesn't exist
        fs::create_dir_all(&brew_package_dir)
            .expect("Failed to create brew_package directory");

        let data_file_path = brew_package_dir.join("config.json");

        FileRepository {
            data_file_path,
        }
    }

    pub fn read_data(&self) -> Result<(String, Vec<ConfigIdentity>), std::io::Error> {

        if !self.data_file_path.exists() {
            return Ok((String::default(), Vec::new()));
        }
    
        let data = fs::read_to_string(&self.data_file_path)?;

        let config_data: ConfigData = match serde_json::from_str(&data) {
            Ok(data) => data,
            Err(_) => {
                return Ok((String::default(), Vec::new()));
            }
        };

        let language = if config_data.language.is_empty() {
            Languages::En.as_str().to_owned()
        } else {
            config_data.language
        };

        Ok((language, config_data.identities))
    }

    fn write_data(
        &self,
        identities: Option<&[ConfigIdentity]>,
        language: Option<&str>,
    ) -> Result<(), std::io::Error> {
        let mut data: serde_json::Value = if self.data_file_path.exists() {
            let existing_data = fs::read_to_string(&self.data_file_path)?;
            serde_json::from_str(&existing_data)?
        } else {
            serde_json::json!({})
        };
    
        if let Some(identities) = identities {
            data["identities"] = serde_json::json!(identities);
        }
    
        if let Some(lang) = language {
            data["language"] = serde_json::json!(lang);
        }
    
        let serialized_data = serde_json::to_string_pretty(&data)?;
        fs::write(&self.data_file_path, serialized_data)
    }
}

impl super::traits::Repository for FileRepository {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        config_path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityRepositoryError> {
        let (_, mut identities) = self.read_data().map_err(|_| AddIdentityRepositoryError::Unknown)?;

        let new_identity = ConfigIdentity {
            alias: alias.clone(),
            hostname: hostname.clone(),
            config_path: config_path.clone(),
        };

        if identities.iter().any(|i| i.alias == alias) {
            return Err(AddIdentityRepositoryError::Conflict);
        }

        identities.push(new_identity.clone());

        if let Err(_) = self.write_data(Some(&identities), None) {
            return Err(AddIdentityRepositoryError::Conflict);
        }

        Ok(new_identity)
    }

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityRepositoryError> {
        let (_, identities) = self.read_data().map_err(|_| FindIdentityRepositoryError::NotFound)?;

        if let Some(identity) = identities.iter().find(|i| i.alias == alias) {
            Ok(identity.clone())
        } else {
            Err(FindIdentityRepositoryError::NotFound)
        }
    }

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError> {
        let (_, identities) = self.read_data().map_err(|_| FindIdentitiesRepositoryError::Unknown)?;

        Ok(identities)
    }

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError> {
        let (_, identities) = self.read_data().map_err(|_| FindIdentitiesRepositoryError::Unknown)?;

        let filtered_identities: Vec<ConfigIdentity> = identities
            .into_iter()
            .filter(|i| i.hostname == hostname)
            .collect();

        Ok(filtered_identities)
    }

    fn delete(&self, alias: Alias) -> Result<(), DeleteIdentityRepositoryError> {
        let (_, mut identities) = self.read_data().map_err(|_| DeleteIdentityRepositoryError::Unknown)?;

        if let Some(index) = identities.iter().position(|i| i.alias == alias) {
            identities.remove(index);

            if let Err(_) = self.write_data(Some(&identities), None) {
                return Err(DeleteIdentityRepositoryError::Unknown);
            }
        } else {
            return Err(DeleteIdentityRepositoryError::NotFound);
        }

        Ok(())
    }

    fn write_language(&self, language: &str) -> Result<(), std::io::Error> {
        self.write_data(None, Some(language))
    }
}


