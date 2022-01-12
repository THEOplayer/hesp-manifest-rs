use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Debug, Clone, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(try_from = "String", into = "String")]
pub enum Uri {
    Absolute(Url),
    Relative(String),
}

impl TryFrom<String> for Uri {
    type Error = url::ParseError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match Url::parse(&value) {
            Ok(url) => Ok(Self::Absolute(url)),
            Err(url::ParseError::RelativeUrlWithoutBase) => Ok(Self::Relative(value)),
            Err(e) => Err(e),
        }
    }
}

impl From<Uri> for String {
    fn from(input: Uri) -> Self {
        match input {
            Uri::Absolute(url) => url.to_string(),
            Uri::Relative(path) => path,
        }
    }
}
