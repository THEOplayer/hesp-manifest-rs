pub use protection::SwitchingSetProtection;

use crate::util::{Entity, EntityIter, EntityIterMut};
use crate::{Result, Track};

mod protection;

pub trait SwitchingSet: Entity {
    type Track: Track;

    fn tracks(&self) -> EntityIter<Self::Track>;
    fn track(&self, id: &str) -> Option<&Self::Track>;
    fn tracks_mut(&mut self) -> EntityIterMut<Self::Track>;
    fn mime_type(&self) -> &str;

    fn validate_active(&self) -> Result<()> {
        for track in self.tracks() {
            track.validate_active()?;
        }
        Ok(())
    }
}
