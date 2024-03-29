use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct AudioMimeType(String);

impl Default for AudioMimeType {
    fn default() -> Self {
        Self("audio/mp4".to_owned())
    }
}

impl TryFrom<String> for AudioMimeType {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if value.starts_with("audio/") {
            Ok(Self(value))
        } else {
            Err(Error::InvalidAudioMime(value))
        }
    }
}

impl AsRef<str> for AudioMimeType {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl From<AudioMimeType> for String {
    fn from(value: AudioMimeType) -> Self {
        value.0
    }
}
