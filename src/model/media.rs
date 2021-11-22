use std::fmt;
use std::str::FromStr;

use crate::{Error, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum MediaType {
    Audio,
    Video,
    Metadata,
}

impl fmt::Display for MediaType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audio => write!(f, "audio"),
            Self::Video => write!(f, "video"),
            Self::Metadata => write!(f, "metadata"),
        }
    }
}

impl FromStr for MediaType {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            "metadata" => Ok(Self::Metadata),
            _ => Err(Error::InvalidMediaType(input.to_owned())),
        }
    }
}
