use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::RelativeUrl;
use crate::{
    DateTime, ManifestVersion, MulticastManifest, Number, PresentationData, StreamType,
    UnicastManifest,
};

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: DateTime,
    pub fallback_poll_rate: Number,
    pub manifest_version: ManifestVersion,
    pub presentations: Vec<PresentationData>,
    #[serde(flatten)]
    pub stream_type: StreamType,
    pub content_base_url: RelativeUrl,
}

impl ManifestData {
    pub fn normalize(&mut self) {
        for presentation in &mut self.presentations {
            presentation.normalize();
        }
    }
}

impl From<UnicastManifest> for ManifestData {
    fn from(input: UnicastManifest) -> Self {
        let mut result = Self {
            creation_date: input.creation_date,
            fallback_poll_rate: input.fallback_poll_rate,
            manifest_version: ManifestVersion::V1_0_0,
            presentations: input
                .presentations
                .into_iter()
                .map(|p| PresentationData::new(p, &input.location))
                .collect(),
            stream_type: input.stream_type,
            content_base_url: RelativeUrl::None,
        };
        result.normalize();
        result
    }
}

impl From<MulticastManifest> for ManifestData {
    fn from(input: MulticastManifest) -> Self {
        input.inner.into()
    }
}
