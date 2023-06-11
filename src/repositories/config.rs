use std::fs;

use crate::domain::entities::{ConfigIdentity, HostName, ConfigPath, Alias};
use std::{path::PathBuf};

use crate::repositories::enums::{FindIdentityRepositoryError, FindIdentitiesRepositoryError, AddIdentityRepositoryError, DeleteIdentityRepositoryError};

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

    fn read_data(&self) -> Result<Vec<ConfigIdentity>, std::io::Error> {
        if !self.data_file_path.exists() {
            // If the file doesn't exist, return an empty vector
            return Ok(Vec::new());
        }

        let data = fs::read_to_string(&self.data_file_path)?;
        let identities: Vec<ConfigIdentity> = serde_json::from_str(&data)?;

        Ok(identities)
    }

    fn write_data(&self, identities: &[ConfigIdentity]) -> Result<(), std::io::Error> {
        let data = serde_json::to_string(identities)?;
        fs::write(&self.data_file_path, data)
    }
}

impl super::traits::Repository for FileRepository {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        config_path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityRepositoryError> {
        let mut identities = self.read_data().map_err(|_| AddIdentityRepositoryError::Unknown)?;

        let new_identity = ConfigIdentity {
            alias: alias.clone(),
            hostname: hostname.clone(),
            config_path: config_path.clone(),
        };

        if identities.iter().any(|i| i.alias == alias) {
            return Err(AddIdentityRepositoryError::Conflict);
        }

        identities.push(new_identity.clone());

        if let Err(_) = self.write_data(&identities) {
            return Err(AddIdentityRepositoryError::Conflict);
        }

        Ok(new_identity)
    }

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityRepositoryError> {
        let identities = self.read_data().map_err(|_| FindIdentityRepositoryError::NotFound)?;

        if let Some(identity) = identities.iter().find(|i| i.alias == alias) {
            Ok(identity.clone())
        } else {
            Err(FindIdentityRepositoryError::NotFound)
        }
    }

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError> {
        let identities = self.read_data().map_err(|_| FindIdentitiesRepositoryError::Unknown)?;

        Ok(identities)
    }

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError> {
        let identities = self.read_data().map_err(|_| FindIdentitiesRepositoryError::Unknown)?;

        let filtered_identities: Vec<ConfigIdentity> = identities
            .into_iter()
            .filter(|i| i.hostname == hostname)
            .collect();

        Ok(filtered_identities)
    }

    fn delete(&self, alias: Alias) -> Result<(), DeleteIdentityRepositoryError> {
        let mut identities = self.read_data().map_err(|_| DeleteIdentityRepositoryError::Unknown)?;

        if let Some(index) = identities.iter().position(|i| i.alias == alias) {
            identities.remove(index);

            if let Err(_) = self.write_data(&identities) {
                return Err(DeleteIdentityRepositoryError::Unknown);
            }
        } else {
            return Err(DeleteIdentityRepositoryError::NotFound);
        }

        Ok(())
    }
}


