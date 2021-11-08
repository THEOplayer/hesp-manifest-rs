use serde::{Deserialize, Serialize};

use crate::*;

pub trait Manifest {
    fn presentations(&self) -> &[Presentation];
    fn presentations_mut(&mut self) -> &mut [Presentation];
    fn content_base_url(&self) -> Option<&RelativeBaseUrl>;
    fn content_base_url_mut(&mut self) -> Option<&mut RelativeBaseUrl>;

    fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations().iter().find(|p| p.id() == id)
    }

    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.presentations_mut().iter_mut().find(|p| p.id() == id)
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0Multicast,
}
