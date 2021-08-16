use std::fmt;
use crate::*;
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum MediaType {
    Audio,
    Video,
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


