use crate::domain::entities::{ConfigIdentity, Alias, HostName, ConfigPath};
use crate::repositories::enums::{AddIdentityError};
use crate::repositories::traits::Repository;
use std::convert::TryFrom;
use std::sync::Arc;

pub struct Request {
    pub alias: String,
    pub hostname: String,
    pub config_path: String,
}

pub struct Response {
    pub alias: String,
    pub hostname: String,
    pub config_path: String,
}

pub enum Error {
    BadRequest,
    Conflict,
    Unknown,
}

pub fn execute(repo: Arc<dyn Repository>, req: Request) -> Result<Response, Error> {
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
            }) => Ok(Response {
                alias: String::from(alias),
                config_path: String::from(config_path),
                hostname: String::from(hostname),
            }),
            Err(AddIdentityError::Conflict) => Err(Error::Conflict),
            Err(AddIdentityError::Unknown) => Err(Error::Unknown),
        },
        _ => Err(Error::BadRequest),
    }
}