use core::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::*;
use crate::model::url::initialization::InitId::Numbered;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitId {
    Now,
    Numbered(u64),
}

impl FromStr for InitId {
    type Err = ParseIntError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "now" => Ok(InitId::Now),
            number => number.parse().map(InitId::Numbered),
        }
    }
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
pub struct InitializationPattern {
    base: Url,
    pattern: String,
}

impl InitializationPattern {
    const INIT_ID_PATTERN: &'static str = "{initId}";

    pub fn new(base: Url, pattern: String) -> Result<Self> {
        base.join(&pattern)?;
        if pattern.contains(Self::INIT_ID_PATTERN) {
            Ok(Self { base, pattern })
        } else {
            Err(Error::InvalidInitializationPattern(pattern))
        }
    }

    pub fn now(&self) -> Url {
        self.init_id(InitId::Now)
    }
    pub fn init_id<I: Into<InitId>>(&self, init_id: I) -> Url {
        let init_id = init_id.into().to_string();
        let rel = self.pattern.replace(Self::INIT_ID_PATTERN, &init_id);
        self.base.join(&rel).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_init_id() {
        assert_eq!("10".parse(), Ok(InitId::Numbered(10)));
        assert_eq!("now".parse(), Ok(InitId::Now));
    }

    #[test]
    fn init_id_to_string() {
        assert_eq!(InitId::Numbered(10).to_string(), "10");
        assert_eq!(InitId::Now.to_string(), "now");
    }
}
