use std::convert::{TryFrom, TryInto};

use serde::{Deserialize, Serialize};

use crate::*;
use super::relative_base::validate_relative;
use url::Url;

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
        validate_segment_id(&value)?;
        Ok(ContinuationPattern(value))
    }
}

impl TryFrom<Url> for ContinuationPattern {
    type Error = Error;
    fn try_from(value: Url) -> Result<Self> {
        let string = value.to_string();
        validate_segment_id(&string)?;
        Ok(ContinuationPattern(string))
    }
}

impl AsRef<str> for ContinuationPattern {
    fn as_ref(&self) -> &str { &self.0 }
}

fn validate_segment_id(value: &str) -> Result<()> {
    if !value.contains("{segmentId}") {
        Err(Error::InvalidContinuationPattern(value.to_owned()))
    } else {
        Ok(())
    }
}