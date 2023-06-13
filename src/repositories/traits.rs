use crate::domain::entities::{Alias, HostName, ConfigIdentity, ConfigPath};

use crate::repositories::enums::{FindIdentityRepositoryError, FindIdentitiesRepositoryError, AddIdentityRepositoryError, DeleteIdentityRepositoryError};

pub trait Repository {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        config_path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityRepositoryError>;

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityRepositoryError>;

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError>;

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesRepositoryError>;

    fn delete(&self, alias: Alias) -> Result<(), DeleteIdentityRepositoryError>;

    fn write_language(&self, language: &str) -> Result<(), std::io::Error>;
}
