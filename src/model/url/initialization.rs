use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};

use crate::*;

use super::relative_base::validate_relative;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct InitializationPattern(String);

impl InitializationPattern {
    pub fn now(&self) -> RelativeBaseUrl {
        self.as_ref().replace("{initId}", "now").try_into().unwrap()
    }
    pub fn init_id(&self, init_id: u64) -> RelativeBaseUrl {
        self.as_ref().replace("{initId}", &init_id.to_string()).try_into().unwrap()
    }
}

impl TryFrom<String> for InitializationPattern {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        validate_relative(&value)?;
        if !value.contains("{initId}") {
            Err(Error::InvalidInitializationPattern(value))
        } else {
            Ok(InitializationPattern(value))
        }
    }
}

impl AsRef<str> for InitializationPattern {
    fn as_ref(&self) -> &str { &self.0 }
}

impl From<InitializationPattern> for String {
    fn from(value: InitializationPattern) -> Self {
        value.0
    }
}