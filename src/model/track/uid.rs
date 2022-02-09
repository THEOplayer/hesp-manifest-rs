use std::fmt;
use std::str::FromStr;
use std::sync::Arc;

use itertools::Itertools;

use crate::{Error, MediaType, Result};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrackUid(Arc<TrackUidData>);

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct TrackUidData {
    pub(crate) presentation_id: String,
    pub(crate) media_type: MediaType,
    pub(crate) switching_set_id: String,
    pub(crate) track_id: String,
}

impl TrackUid {
    pub fn new(
        presentation_id: String,
        media_type: MediaType,
        switching_set_id: String,
        track_id: String,
    ) -> Self {
        let data = TrackUidData {
            presentation_id,
            media_type,
            switching_set_id,
            track_id,
        };
        Self(Arc::new(data))
    }
    #[inline]
    pub fn presentation_id(&self) -> &str {
        &self.0.presentation_id
    }
    #[inline]
    pub fn media_type(&self) -> MediaType {
        self.0.media_type
    }
    #[inline]
    pub fn switching_set_id(&self) -> &str {
        &self.0.switching_set_id
    }
    #[inline]
    pub fn track_id(&self) -> &str {
        &self.0.track_id
    }
}

impl fmt::Display for TrackUid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.presentation_id(),
            self.media_type(),
            self.switching_set_id(),
            self.track_id(),
        )
    }
}

impl FromStr for TrackUid {
    type Err = Error;
    fn from_str(input: &str) -> Result<Self> {
        let (presentation_id, media_type, switching_set_id, track_id) = input
            .split('/')
            .collect_tuple()
            .ok_or_else(|| Error::InvalidTrackPath(input.to_owned()))?;
        Ok(Self::new(
            presentation_id.to_owned(),
            media_type.parse()?,
            switching_set_id.to_owned(),
            track_id.to_owned(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_track_path() -> Result<()> {
        let uid: TrackUid = "main-pres/video/main/720p".parse()?;

        assert_eq!(
            uid,
            TrackUid::new(
                "main-pres".to_owned(),
                MediaType::Video,
                "main".to_owned(),
                "720p".to_owned(),
            )
        );
        Ok(())
    }
}
