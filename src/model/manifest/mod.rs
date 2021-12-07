use url::Url;

pub use data::ManifestData;
pub use multicast::*;
pub use stream::*;
pub use unicast::*;
pub use version::ManifestVersion;

use crate::util::{EntityIter, EntityIterMut};
use crate::{Presentation, Result};

mod data;
mod multicast;
mod stream;
mod unicast;
mod version;

pub trait Manifest {
    fn new(location: Url, data: ManifestData) -> Result<Self>
    where
        Self: Sized;
    fn presentations(&self) -> EntityIter<Presentation>;
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation>;
    fn presentation(&self, id: &str) -> Option<&Presentation>;
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation>;
    fn stream_type(&self) -> &StreamType;

    fn from_json(location: Url, json: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let data: ManifestData = serde_path_to_error::deserialize(deserializer)?;
        Self::new(location, data)
    }
}
