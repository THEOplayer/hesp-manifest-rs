use crate::util::{Entity, EntityIter, EntityIterMut};
use crate::*;

pub trait SwitchingSet: Entity<Id = str> {
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
