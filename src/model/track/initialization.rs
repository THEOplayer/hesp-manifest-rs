use core::fmt;
use std::num::ParseIntError;
use std::str::FromStr;

use url::Url;

use crate::util::Uri;
use crate::{Address, Error, Result, Track, UrlPattern};

pub trait Initialization: Track {
    fn initialization_pattern(&self) -> &InitializationPattern;
    fn initialization_pattern_mut(&mut self) -> &mut InitializationPattern;
    fn active_sequence_number(&self) -> Option<u64>;

    fn validate_active(&self) -> Result<()> {
        if self.active_sequence_number().is_none() {
            Err(Error::MissingActiveSequenceNumber(self.id().to_owned()))
        } else {
            Ok(())
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum InitId {
    Now,
    Numbered(u64),
}

impl From<u64> for InitId {
    fn from(id: u64) -> Self {
        InitId::Numbered(id)
    }
}

impl FromStr for InitId {
    type Err = ParseIntError;

    fn from_str(input: &str) -> std::result::Result<Self, Self::Err> {
        match input {
            "now" => Ok(Self::Now),
            number => number.parse().map(Self::Numbered),
        }
    }
}

impl fmt::Display for InitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            InitId::Now => write!(f, "now"),
            InitId::Numbered(id) => id.fmt(f),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InitializationPattern(UrlPattern);

impl InitializationPattern {
    const INIT_ID_PATTERN: &'static str = "{initId}";

    pub fn new(address: Address, pattern: String) -> Result<Self> {
        UrlPattern::new(address, pattern, Self::INIT_ID_PATTERN).map(Self)
    }

    pub fn now(&self) -> Url {
        self.init_id(InitId::Now)
    }

    pub fn init_id<I: Into<InitId>>(&self, init_id: I) -> Url {
        self.0.resolve(&init_id.into().to_string()).unwrap()
    }

    pub fn base_url(&self) -> Option<&Uri> {
        self.0.base_url()
    }

    pub fn into_pattern(self) -> String {
        self.0.into_pattern()
    }

    pub fn into_full_pattern(self) -> String {
        self.0.into_pattern_including_address()
    }

    pub fn set_pattern(&mut self, pattern: String) -> Result<()> {
        self.0.set_pattern(pattern)
    }

    pub fn set_base_url(&mut self, base_url: Option<Uri>) -> Result<()> {
        self.0.set_base_url(base_url)
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
