use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum MediaType {
    Audio,
    Video,
}

impl MediaType {
    pub fn content_type(self) -> &'static str {
        match self {
            MediaType::Audio => "audio/mp4",
            MediaType::Video => "video/mp4",
        }
    }
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MediaType::Audio => write!(f, "audio"),
            MediaType::Video => write!(f, "video"),
        }
    }
}

impl FromStr for MediaType {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "audio" => Ok(MediaType::Audio),
            "video" => Ok(MediaType::Video),
            _ => Err(Error::InvalidMediaType(input.to_owned())),
        }
    }
}
