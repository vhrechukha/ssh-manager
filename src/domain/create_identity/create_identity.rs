use crate::domain::entities::{ConfigIdentity, Alias, HostName, ConfigPath};
use crate::repositories::enums::{AddIdentityError};
use crate::repositories::traits::Repository;
use std::convert::TryFrom;
use std::sync::Arc;

use super::enums::CreateIdentityError;
use super::structs::{CreateIdentityRequest, CreateIdentityResponse};

pub fn execute(repo: Arc<dyn Repository>, req: CreateIdentityRequest) -> Result<CreateIdentityResponse, CreateIdentityError> {
    match (
        Alias::try_from(req.alias),
        HostName::try_from(req.hostname),
        ConfigPath::try_from(req.config_path),
    ) {
        (Ok(alias), Ok(hostname), Ok(path)) => match repo.add(alias, hostname, path) {
            Ok(ConfigIdentity {
                alias,
                hostname,
                config_path,
            }) => Ok(CreateIdentityResponse {
                alias: String::from(alias),
                config_path: String::from(config_path),
                hostname: String::from(hostname),
            }),
            Err(AddIdentityError::Conflict) => Err(CreateIdentityError::Conflict),
            Err(AddIdentityError::Unknown) => Err(CreateIdentityError::Unknown),
        },
        _ => Err(CreateIdentityError::BadRequest),
    }
}