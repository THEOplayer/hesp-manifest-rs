use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::{RelativeUrl, UInt};
use crate::{legacy, DateTime, MulticastManifest, PresentationData, StreamType, UnicastManifest};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestDeserialize {
    #[serde(rename = "1.0.0")]
    V1_0_0(legacy::v1_0_0::ManifestData),
    #[serde(rename = "1.1.0")]
    V1_1_0(ManifestData),
    #[serde(rename = "1.0.0-multicast")]
    V1_1_0Multicast(ManifestData),
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestSerialize {
    #[serde(rename = "1.1.0")]
    V1_1_0(ManifestData),
    #[serde(rename = "1.0.0-multicast")]
    V1_1_0Multicast(ManifestData),
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: DateTime,
    pub fallback_poll_rate: UInt,
    pub presentations: Vec<PresentationData>,
    #[serde(flatten)]
    pub stream_type: StreamType,
    #[serde(skip_serializing_if = "RelativeUrl::is_none")]
    pub content_base_url: RelativeUrl,
}

impl ManifestData {
    pub fn normalize(&mut self) {
        for presentation in &mut self.presentations {
            presentation.normalize();
        }
    }
}

impl From<MulticastManifest> for ManifestSerialize {
    fn from(input: MulticastManifest) -> Self {
        Self::V1_1_0Multicast(ManifestData::from(input))
    }
}

impl From<UnicastManifest> for ManifestSerialize {
    fn from(input: UnicastManifest) -> Self {
        Self::V1_1_0(ManifestData::from(input))
    }
}

impl From<UnicastManifest> for ManifestData {
    fn from(input: UnicastManifest) -> Self {
        let mut result = Self {
            creation_date: input.creation_date,
            fallback_poll_rate: input.fallback_poll_rate.into(),
            presentations: input
                .presentations
                .into_iter()
                .map(PresentationData::from)
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
