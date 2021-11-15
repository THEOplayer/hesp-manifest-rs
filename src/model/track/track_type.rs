use std::fmt;
use std::str::FromStr;
use crate::*;

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub enum TrackType {
    Audio,
    Video,
    Metadata
}


impl fmt::Display for TrackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Audio => write!(f, "audio"),
            Self::Video => write!(f, "video"),
            Self::Metadata => write!(f, "metadata"),
        }
    }
}

impl FromStr for TrackType {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        match input {
            "audio" => Ok(Self::Audio),
            "video" => Ok(Self::Video),
            "metadata" => Ok(Self::Metadata),
            _ => Err(Error::InvalidTrackType(input.to_owned())),
        }
    }
}
