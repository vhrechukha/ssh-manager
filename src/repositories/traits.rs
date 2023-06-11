use crate::domain::entities::{Alias, HostName, ConfigIdentity, ConfigPath};

use super::config::{AddIdentityError, FindIdentityError, FindIdentitiesError, DeleteError};

pub trait Repository {
    fn add(
        &self,
        alias: Alias,
        hostname: HostName,
        config_path: ConfigPath,
    ) -> Result<ConfigIdentity, AddIdentityError>;

    fn find_one(&self, alias: Alias) -> Result<ConfigIdentity, FindIdentityError>;

    fn find_all(&self) -> Result<Vec<ConfigIdentity>, FindIdentitiesError>;

    fn find_all_with_hostname(&self, hostname: HostName) -> Result<Vec<ConfigIdentity>, FindIdentitiesError>;

    fn delete(&self, alias: Alias) -> Result<(), DeleteError>;
}
