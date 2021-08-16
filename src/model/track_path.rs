use crate::*;
use std::fmt;
use std::str::FromStr;
use itertools::Itertools;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrackPath {
    presentation_id: String,
    switching_set_id: String,
    media_type: MediaType,
    track_id: String,
}

impl TrackPath {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        media_type: MediaType,
        track_id: String,
    ) -> Self {
        Self { presentation_id, switching_set_id, media_type, track_id }
    }

    pub fn presentation_id(&self) -> &str { &self.presentation_id }
    pub fn switching_set_id(&self) -> &str { &self.switching_set_id }
    pub fn media_type(&self) -> MediaType { self.media_type }
    pub fn track_id(&self) -> &str { &self.track_id }
}

impl fmt::Display for TrackPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}/{}/{}/{}",
            self.presentation_id, self.switching_set_id, self.media_type, self.track_id,
        )
    }
}

impl FromStr for TrackPath {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let (presentation_id, switching_set_id, media_type, track_id) = input
            .split('/')
            .collect_tuple()
            .ok_or_else(|| Error::InvalidTrackPath(input.to_owned()))?;
        Ok(TrackPath::new(
            presentation_id.to_owned(),
            switching_set_id.to_owned(),
            media_type.parse()?,
            track_id.to_owned()
        ))
    }
}



