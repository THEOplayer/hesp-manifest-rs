use crate::Entity;

use super::*;

pub trait SwitchingSet: Entity<Id=str> {
    type Track: Track;
    fn tracks(&self) -> &[Self::Track];
    fn tracks_mut(&mut self) -> &mut [Self::Track];
    fn base_url(&self) -> &Option<RelativeBaseUrl>;
    fn mime_type(&self) -> &str;
}

pub trait MediaSwitchingSet: SwitchingSet {}