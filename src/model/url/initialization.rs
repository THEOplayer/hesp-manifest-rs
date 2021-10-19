use core::fmt;
use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};

use crate::*;

use super::relative_base::validate_relative;
use crate::model::url::initialization::InitId::Numbered;
use url::Url;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum InitId {
    Now,
    Numbered(u64),
}

impl fmt::Display for InitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitId::Now => write!(f, "now"),
            Numbered(id) => id.fmt(f),
        }
    }
}

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String")]
pub struct InitializationPattern(String);

impl InitializationPattern {
    pub fn now(&self) -> RelativeBaseUrl {
        self.init_id(InitId::Now)
    }
    pub fn init_id<I: Into<InitId>>(&self, init_id: I) -> RelativeBaseUrl {
        self.as_ref()
            .replace("{initId}", &init_id.into().to_string())
            .try_into()
            .unwrap()
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
    fn as_ref(&self) -> &str {
        &self.0
    }
}

fn validate_init_id(value: &str) -> Result<()> {
    if !value.contains("{initId}") {
        Err(Error::InvalidInitializationPattern(value.to_owned()))
    } else {
        Ok(())
    }
}
