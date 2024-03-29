use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::data::PresentationData;
use crate::util::{Timestamp, UInt, Uri};
use crate::{Manifest, StreamType};

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

impl From<Manifest> for ManifestData {
    fn from(input: Manifest) -> Self {
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
