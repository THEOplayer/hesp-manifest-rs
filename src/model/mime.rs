use std::convert::TryFrom;

use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct AudioMimeType(String);

#[derive(Debug, Deserialize, Clone, Serialize)]
#[serde(try_from = "String", into = "String")]
pub struct VideoMimeType(String);

impl Default for AudioMimeType {
    fn default() -> Self { Self("audio/mp4".to_string()) }
}

impl TryFrom<String> for AudioMimeType {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if !value.starts_with("audio/") {
            Err(Error::InvalidAudioMime(value))
        } else {
            Ok(AudioMimeType(value))
        }
    }
}

impl AsRef<str> for AudioMimeType {
    fn as_ref(&self) -> &str { &self.0 }
}

impl From<AudioMimeType> for String {
    fn from(value: AudioMimeType) -> Self {
        value.0
    }
}

impl Default for VideoMimeType {
    fn default() -> Self { Self("video/mp4".to_string()) }
}

impl TryFrom<String> for VideoMimeType {
    type Error = Error;
    fn try_from(value: String) -> Result<Self> {
        if !value.starts_with("video/") {
            Err(Error::InvalidAudioMime(value))
        } else {
            Ok(VideoMimeType(value))
        }
    }
}

impl AsRef<str> for VideoMimeType {
    fn as_ref(&self) -> &str { &self.0 }
}

impl From<VideoMimeType> for String {
    fn from(value: VideoMimeType) -> Self {
        value.0
    }
}