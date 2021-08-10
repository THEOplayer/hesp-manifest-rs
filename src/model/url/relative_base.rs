use std::convert::TryFrom;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::*;

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(try_from = "String", into = "String")]
pub struct RelativeBaseUrl(String);

pub trait RelativeUrl {
    fn resolve(&self, url: &Url) -> Url;
}

impl RelativeUrl for RelativeBaseUrl {
    fn resolve(&self, url: &Url) -> Url {
        url.join(self.as_ref()).unwrap()
    }
}

impl RelativeUrl for Option<RelativeBaseUrl> {
    fn resolve(&self, url: &Url) -> Url {
        if let Some(relative_url) = self {
            relative_url.resolve(url)
        } else {
            url.clone()
        }
    }
}

impl TryFrom<String> for RelativeBaseUrl {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        validate_relative(&value)?;
        Ok(RelativeBaseUrl(value))
    }
}

impl AsRef<str> for RelativeBaseUrl {
    fn as_ref(&self) -> &str { &self.0 }
}

impl From<RelativeBaseUrl> for String {
    fn from(value: RelativeBaseUrl) -> Self { value.0 }
}

lazy_static! {
    static ref TEST_BASE: Url = Url::parse("https://theoplayer.com").unwrap();
}

// check if is valid relative url by checking against a base url
pub(super) fn validate_relative(url: &str) -> Result<()> {
    TEST_BASE.join(url)?;
    Ok(())
}