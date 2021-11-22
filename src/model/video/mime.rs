use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct VideoMimeType(String);

impl Default for VideoMimeType {
    fn default() -> Self {
        Self("video/mp4".to_owned())
    }
}

impl TryFrom<String> for VideoMimeType {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if value.starts_with("video/") {
            Ok(Self(value))
        } else {
            Err(Error::InvalidAudioMime(value))
        }
    }
}

impl AsRef<str> for VideoMimeType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<VideoMimeType> for String {
    fn from(value: VideoMimeType) -> Self {
        value.0
    }
}
