use serde::{Deserialize, Serialize};
use url::Url;

use crate::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(from = "Option<String>", into = "Option<String>")]
pub enum RelativeUrl {
    Absolute(Url),
    Path(String),
    None,
}

impl RelativeUrl {
    pub fn resolve(&self, url: &Url) -> Result<Url> {
        Ok(match self {
            Self::Absolute(url) => url.clone(),
            Self::Path(path) => url.join(path)?,
            Self::None => url.clone(),
        })
    }

    // pub fn make_relative(&mut self, url: &Url) {
    //     if let Self::Absolute(absolute) = self {
    //         if let Some(relative) = url.make_relative(&absolute) {
    //             *self = relative.into();
    //         }
    //     }
    // }
}

impl From<Option<String>> for RelativeUrl {
    fn from(input: Option<String>) -> Self {
        match input {
            None => Self::None,
            Some(input) => match Url::parse(&input) {
                Ok(url) => Self::Absolute(url),
                Err(_) => Self::Path(input),
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
