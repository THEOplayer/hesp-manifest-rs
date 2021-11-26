use url::Url;

pub use data::*;
pub use multicast::*;
pub use stream::*;
pub use unicast::*;

use crate::util::{EntityIter, EntityIterMut};
use crate::{Presentation, Result};

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

    fn from_json(location: Url, json: &str) -> Result<Self>;
}
