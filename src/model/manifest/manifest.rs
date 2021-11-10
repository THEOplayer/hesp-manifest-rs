use serde::{Deserialize, Serialize};

use crate::util::{EntityIter, EntityIterMut};
use crate::*;

pub trait Manifest {
    fn presentations(&self) -> EntityIter<Presentation>;
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation>;
    fn presentation(&self, id: &str) -> Option<&Presentation>;
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation>;
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0Multicast,
}
