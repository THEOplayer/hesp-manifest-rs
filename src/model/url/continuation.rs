use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};

use crate::*;
use super::relative_base::validate_relative;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String")]
pub struct ContinuationPattern(String);

impl ContinuationPattern {
    pub fn segment(&self, id: u64) -> RelativeBaseUrl {
        self.as_ref().replace("{segmentId}", &id.to_string()).try_into().unwrap()
    }
}

impl TryFrom<String> for ContinuationPattern {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        validate_relative(&value)?;
        if !value.contains("{segmentId}") {
            Err(Error::InvalidContinuationPattern(value))
        } else {
            Ok(ContinuationPattern(value))
        }
    }
}

impl AsRef<str> for ContinuationPattern {
    fn as_ref(&self) -> &str { &self.0 }
}
