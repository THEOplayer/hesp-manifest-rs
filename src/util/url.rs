use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "Option<String>", into = "Option<String>")]
pub enum RelativeUrl {
    Absolute(Url),
    Path(String),
    None,
}

impl RelativeUrl {
    pub const fn is_none(&self) -> bool {
        matches!(self, Self::None)
    }
}

impl TryFrom<Option<String>> for RelativeUrl {
    type Error = url::ParseError;

    fn try_from(value: Option<String>) -> Result<Self, Self::Error> {
        match value {
            None => Ok(Self::None),
            Some(input) => match Url::parse(&input) {
                Ok(url) => Ok(Self::Absolute(url)),
                Err(url::ParseError::RelativeUrlWithoutBase) => Ok(Self::Path(input)),
                Err(e) => Err(e),
            },
        }
    }
}

impl From<RelativeUrl> for Option<String> {
    fn from(input: RelativeUrl) -> Self {
        match input {
            RelativeUrl::Absolute(url) => Some(url.to_string()),
            RelativeUrl::Path(path) => Some(path),
            RelativeUrl::None => None,
        }
    }
}
