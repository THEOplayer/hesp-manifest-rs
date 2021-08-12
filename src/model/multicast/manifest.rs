use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;
use crate::model::manifest::LiveStream;

validate_on_deserialize!(MulticastManifest);
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct MulticastManifest {
    creation_date: DateTime,
    fallback_poll_rate: Number,
    manifest_version: MulticastManifestVersion,
    presentations: EntityVec<Presentation>,
    stream_type: MulticastStreamType,
    #[serde(flatten)]
    live_data: LiveStream,
    content_base_url: Option<RelativeBaseUrl>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
enum MulticastManifestVersion {
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum MulticastStreamType {
    Live,
}

impl MulticastManifest {
    pub fn presentations(&self) -> &[Presentation] { &self.presentations }
    pub fn content_base_url(&self) -> &Option<RelativeBaseUrl> { &self.content_base_url }
    pub fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.iter().find(|p| p.id() == id)
    }
}

impl Validate for MulticastManifest {
    fn validate(&self) -> Result<()> {
        let active_id = &self.live_data.active_presentation;
        self.presentation(active_id)
            .ok_or_else(|| Error::InvalidActivePresentationId(active_id.to_owned()))?
            .validate_active()
    }
}
