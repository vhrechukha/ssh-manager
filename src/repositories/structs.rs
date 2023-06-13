use serde::{Serialize, Deserialize};

use crate::domain::entities::ConfigIdentity;

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigData {
    pub language: String,
    pub identities: Vec<ConfigIdentity>,
}