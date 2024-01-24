use chrono::{DateTime, FixedOffset};
use url::Url;

pub use base::*;
pub use data::*;
pub use stream::*;

use crate::util::{EntityIter, EntityIterMut};
use crate::{InitializableTrack, Presentation, Result, Track, TrackUid};

mod base;
mod data;
mod stream;

pub trait Manifest: Sized {
    fn new(location: Url, data: ManifestData) -> Result<Self>;
    fn presentations(&self) -> EntityIter<Presentation>;
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation>;
    fn presentation(&self, id: &str) -> Option<&Presentation>;
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation>;
    fn stream_type(&self) -> &StreamType;

    fn track(&self, track_uid: &TrackUid) -> Option<&dyn Track> {
        self.presentation(track_uid.presentation_id())?.track(
            track_uid.media_type(),
            track_uid.switching_set_id(),
            track_uid.track_id(),
        )
    }

    fn track_mut(&mut self, track_uid: &TrackUid) -> Option<&mut dyn Track> {
        self.presentation_mut(track_uid.presentation_id())?
            .track_mut(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    fn initializable_track(&self, track_uid: &TrackUid) -> Option<&dyn InitializableTrack> {
        self.presentation(track_uid.presentation_id())?
            .initializable_track(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    fn initializable_track_mut(
        &mut self,
        track_uid: &TrackUid,
    ) -> Option<&mut dyn InitializableTrack> {
        self.presentation_mut(track_uid.presentation_id())?
            .initializable_track_mut(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    fn creation_date(&self) -> DateTime<FixedOffset>;

    fn fallback_poll_rate(&self) -> u64;

    fn from_json(location: Url, json: &str) -> Result<Self>;
}
