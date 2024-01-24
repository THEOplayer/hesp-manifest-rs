use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::manifest::base::BaseManifest;
use crate::util::{Timestamp, UInt, Uri};
use crate::{legacy, PresentationData, StreamType, UnicastManifest};

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestDeserialize {
    #[serde(rename = "1.0.0")]
    V1_0_0(legacy::v1_0_0::ManifestData),
    #[serde(rename = "1.1.0")]
    V1_1_0(legacy::v1_1_0::ManifestData),
    #[serde(rename = "2.0.0")]
    V2_0_0(ManifestData), //TODO is it v2.0.0 or v1.2.0?
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestSerialize {
    #[serde(rename = "2.0.0")]
    V2_0_0(ManifestData), //TODO is it v2.0.0 or v1.2.0?
}

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: Timestamp,
    pub fallback_poll_rate: UInt,
    pub presentations: Vec<PresentationData>,
    #[serde(flatten)]
    pub stream_type: StreamType,
    pub content_base_url: Option<Uri>,
}

impl ManifestData {
    pub fn normalize(&mut self) {
        for presentation in &mut self.presentations {
            presentation.normalize();
        }
    }
}

impl From<BaseManifest> for ManifestData {
    fn from(input: BaseManifest) -> Self {
        let mut result = Self {
            creation_date: input.creation_date.into(),
            fallback_poll_rate: input.fallback_poll_rate.into(),
            presentations: input
                .presentations
                .into_iter()
                .map(PresentationData::from)
                .collect(),
            stream_type: input.stream_type,
            content_base_url: None,
        };
        result.normalize();
        result
    }
}

impl From<UnicastManifest> for ManifestSerialize {
    fn from(input: UnicastManifest) -> Self {
        Self::V2_0_0(ManifestData::from(input))
    }
}

impl From<UnicastManifest> for ManifestData {
    fn from(input: UnicastManifest) -> Self {
        input.inner.into()
    }
}
