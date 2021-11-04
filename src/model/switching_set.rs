use crate::Entity;
use crate::Result;

use super::*;

pub trait SwitchingSet: Entity<Id=str> {
    type Track: Track;
    fn tracks(&self) -> &[Self::Track];
    fn track(&self, id: &str) -> Option<&Self::Track>;
    fn tracks_mut(&mut self) -> &mut [Self::Track];
    fn base_url(&self) -> &Option<RelativeBaseUrl>;
    fn base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl>;
    fn mime_type(&self) -> &str;
}

pub trait MediaSwitchingSet: SwitchingSet<Track=<Self as MediaSwitchingSet>::MediaTrack> {
    type MediaTrack: MediaTrack;
    const MEDIA_TYPE: MediaType;

    fn validate_active(&self) -> Result<()> {
        for track in self.tracks() {
            track.validate_active()?
        }
        Ok(())
    }
}
