use serde::{Deserialize, Serialize, Serializer};

use crate::domain::entities::{ConfigIdentity, HostName, ConfigPath, Alias};
use std::sync::{Mutex, MutexGuard};

pub enum AddIdentityError {
    Conflict,
    Unknown,
}

pub enum FindIdentityError {
    NotFound,
    Unknown,
}

pub enum FindIdentitiesError {
    Unknown,
}

pub enum DeleteError {
    Unknown,
    NotFound,
}

pub trait Repository: Send + Sync {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityError>;

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityError>;

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesError>;

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesError>;

    fn delete(&self, alias: Alias) -> Result<(), DeleteError>;
}

pub struct InMemoryRepository {
    error: bool,
    identities: Mutex<Vec<ConfigIdentity>>,
}

#[derive(Clone, Deserialize, Serialize, Debug)]
struct ConfigIdentities {
    identities: Vec<ConfigIdentity>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        let path_to_config = "config.json";
        let config = std::path::Path::new(path_to_config).exists();

        if !config {
            let identities_file: ConfigIdentities = ConfigIdentities {
                identities: vec![],
            };

            std::fs::write(
                path_to_config,
                serde_json::to_string_pretty(&identities_file).unwrap(),
            )
            .unwrap();
        }

        let config = {
            // Load the first file into a string.
            let text = std::fs::read_to_string(path_to_config).unwrap();
    
            // Parse the string into a dynamically-typed JSON structure.
            serde_json::from_str::<ConfigIdentities>(&text).unwrap()
        };

        let identities = Mutex::new(config.identities);

        Self {
            error: false,
            identities,
        }
    }
}

pub struct MutexSerializer<'a, T>(pub MutexGuard<'a, T>);

impl<T: Serialize> Serialize for MutexSerializer<'_, T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.0.serialize(serializer)
    }
}

impl Repository for InMemoryRepository {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityError> {
        if self.error {
            return Err(AddIdentityError::Unknown);
        }

        let mut lock = match self.identities.lock() {
            Ok(lock) => lock,
            _ => return Err(AddIdentityError::Unknown),
        };

        if lock.iter().any(|identity| identity.alias == alias) {
            return Err(AddIdentityError::Conflict);
        }

        let identity = ConfigIdentity::new(alias, hostname, path);
        lock.push(identity.clone());

        let final_result = { ConfigIdentities {
            identities: lock.to_vec(),
        } };
        std::fs::write(
            "config.json",
            serde_json::to_string_pretty(&final_result).unwrap(),
        )
        .unwrap();
        Ok(identity)
    }

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityError> {
        if self.error {
            return Err(FindIdentityError::Unknown);
        }

        let lock = match self.identities.lock() {
            Ok(lock) => lock,
            _ => return Err(FindIdentityError::Unknown),
        };

        match lock.iter().find(|identity| identity.alias == alias) {
            Some(identity) => Ok(identity.clone()),
            None => Err(FindIdentityError::NotFound),
        }
    }

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesError> {
        if self.error {
            return Err(FindIdentitiesError::Unknown);
        }

        let lock = match self.identities.lock() {
            Ok(lock) => lock,
            _ => return Err(FindIdentitiesError::Unknown),
        };

        let identities = lock.to_vec();
        Ok(identities)
    }

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesError> {
        if self.error {
            return Err(FindIdentitiesError::Unknown);
        }

        let lock = match self.identities.lock() {
            Ok(lock) => lock,
            _ => return Err(FindIdentitiesError::Unknown),
        };

        let identities = lock
            .to_vec()
            .into_iter()
            .filter(|identity| identity.hostname == hostname)
            .collect();

        Ok(identities)
    }

    fn delete(&self, alias: Alias) -> Result<(), DeleteError> {
        if self.error {
            return Err(DeleteError::Unknown);
        }

        let mut lock = match self.identities.lock() {
            Ok(lock) => lock,
            _ => return Err(DeleteError::Unknown),
        };

        let index = match lock.iter().position(|p| p.alias == alias) {
            Some(index) => index,
            None => return Err(DeleteError::NotFound),
        };

        lock.remove(index);

        let final_result = { ConfigIdentities {
            identities: lock.to_vec(),
        } };

        std::fs::write(
            "config.json",
            serde_json::to_string_pretty(&final_result).unwrap(),
        )
        .unwrap();

        Ok(())
    }
}