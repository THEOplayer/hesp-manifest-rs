use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};

use crate::*;

use super::relative_base::validate_relative;
use url::Url;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String")]
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
        validate_init_id(&value)?;
        Ok(InitializationPattern(value))
    }
}

impl TryFrom<Url> for InitializationPattern {
    type Error = Error;
    fn try_from(value: Url) -> Result<Self> {
        let string = value.to_string();
        validate_init_id(&string)?;
        Ok(InitializationPattern(string))
    }
}

impl AsRef<str> for InitializationPattern {
    fn as_ref(&self) -> &str { &self.0 }
}

fn validate_init_id(value: &str) -> Result<()> {
    if !value.contains("{initId}") {
        Err(Error::InvalidInitializationPattern(value.to_owned()))
    } else {
        Ok(())
    }
}