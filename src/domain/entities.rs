use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct HostName(pub String);

impl TryFrom<String> for HostName {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<HostName> for String {
    fn from(n: HostName) -> Self {
        n.0
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ConfigPath(String);

impl TryFrom<String> for ConfigPath {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<ConfigPath> for String {
    fn from(n: ConfigPath) -> Self {
        n.0
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
pub struct Alias(String);

impl TryFrom<String> for Alias {
    type Error = ();

    fn try_from(n: String) -> Result<Self, Self::Error> {
        if n.is_empty() {
            Err(())
        } else {
            Ok(Self(n))
        }
    }
}

impl From<Alias> for String {
    fn from(n: Alias) -> Self {
        n.0
    }
}


#[derive(Clone, Deserialize, Serialize, Debug)]
pub struct ConfigIdentity {
    pub hostname: HostName,
    pub config_path: ConfigPath,
    pub alias: Alias,
}

impl ConfigIdentity {
    pub fn new(alias: Alias, hostname: HostName, config_path: ConfigPath) -> Self {
        Self {
            alias,
            hostname,
            config_path,
        }
    }
}
