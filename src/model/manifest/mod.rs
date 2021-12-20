use url::Url;

pub use data::*;
pub use multicast::*;
pub use stream::*;
pub use unicast::*;

use crate::util::{EntityIter, EntityIterMut};
use crate::{Presentation, Result, Track, TrackUid};

mod data;
mod multicast;
mod stream;
mod unicast;

pub trait Manifest: Sized {
    fn new(location: Url, data: ManifestData) -> Result<Self>
    where
        Self: Sized;
    fn presentations(&self) -> EntityIter<Presentation>;
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation>;
    fn presentation(&self, id: &str) -> Option<&Presentation>;
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation>;
    fn stream_type(&self) -> &StreamType;

    fn track_by_uid(&self, track_uid: &TrackUid) -> Option<&dyn Track> {
        match self.presentation(track_uid.presentation_id()) {
            None => None,
            Some(presentation) => presentation.track_by_uid(track_uid),
        }
    }

    fn track_mut_by_uid(&mut self, track_uid: &TrackUid) -> Option<&mut dyn Track> {
        match self.presentation_mut(track_uid.presentation_id()) {
            None => None,
            Some(presentation) => presentation.track_mut_by_uid(track_uid),
        }
    }

    fn from_json(location: Url, json: &str) -> Result<Self>;
}
