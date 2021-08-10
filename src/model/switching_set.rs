use super::*;
use crate::Entity;

pub trait SwitchingSet: Entity<Id=str> {
    type Track: Track;
    fn tracks(&self) -> &[Self::Track];
    fn base_url(&self) -> &Option<RelativeBaseUrl>;
    fn mime_type(&self) -> &str;
}

pub trait MediaSwitchingSet: SwitchingSet<Track=Self::MediaTrack> {
    type MediaTrack: MediaTrack;
}