pub use protection::SwitchingSetProtection;

use crate::util::{Entity, EntityIter, EntityIterMut};
use crate::{MediaTrack, MediaType, Track};

mod protection;

pub trait SwitchingSet: Entity {
    type Track: Track;
    fn tracks(&self) -> EntityIter<Self::Track>;
    fn track(&self, id: &str) -> Option<&Self::Track>;
    fn tracks_mut(&mut self) -> EntityIterMut<Self::Track>;
    fn mime_type(&self) -> &str;
}

pub trait MediaSwitchingSet: SwitchingSet<Track = <Self as MediaSwitchingSet>::MediaTrack> {
    type MediaTrack: MediaTrack;
    const MEDIA_TYPE: MediaType;
}
