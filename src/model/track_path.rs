use crate::*;

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