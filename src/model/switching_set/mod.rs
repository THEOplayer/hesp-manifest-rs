pub use protection::SwitchingSetProtection;

use crate::util::{Entity, EntityIter, EntityIterMut};
use crate::{MediaType, Result, Track, ValidateTrack};

mod protection;

pub trait SwitchingSet: Entity {
    type Track: Track;

    fn media_type(&self) -> MediaType;
    fn tracks(&self) -> EntityIter<Self::Track>;
    fn track(&self, id: &str) -> Option<&Self::Track>;
    fn track_mut(&mut self, id: &str) -> Option<&mut Self::Track>;
    fn tracks_mut(&mut self) -> EntityIterMut<Self::Track>;
    fn mime_type(&self) -> &str;
}

pub(crate) trait ValidateSwitchingSet<T: ValidateTrack>: SwitchingSet<Track = T> {
    fn validate_active(&self) -> Result<()> {
        for track in self.tracks() {
            track.validate_active()?;
        }
        Ok(())
    }
}
