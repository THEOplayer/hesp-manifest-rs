use crate::{MediaType, Presentation, SwitchingSet, Track, Entity};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct TrackPath {
    presentation_id: String,
    switching_set_id: String,
    media_type: MediaType,
    track_id: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy)]
pub struct TrackPathRef<'a> {
    presentation_id: &'a str,
    switching_set_id: &'a str,
    media_type: MediaType,
    track_id: &'a str
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
}

impl<'a> TrackPathRef<'a> {
    pub fn new(
        presentation_id: &'a str,
        switching_set_id: &'a str,
        media_type: MediaType,
        track_id: &'a str,
    ) -> Self {
        Self { presentation_id, switching_set_id, media_type, track_id }
    }
}