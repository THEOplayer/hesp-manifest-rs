use serde::Deserialize;

use crate::util::{UInt, Uri};
use crate::{DateTime, StreamType};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: DateTime,
    pub fallback_poll_rate: UInt,
    pub presentations: Vec<super::PresentationData>,
    #[serde(flatten)]
    pub stream_type: StreamType,
    pub content_base_url: Option<Uri>,
}

impl From<ManifestData> for crate::ManifestData {
    fn from(input: ManifestData) -> Self {
        Self {
            creation_date: input.creation_date,
            fallback_poll_rate: input.fallback_poll_rate,
            presentations: input
                .presentations
                .into_iter()
                .map(crate::PresentationData::from)
                .collect(),
            stream_type: input.stream_type,
            content_base_url: input.content_base_url,
            multicast_metadata: None,
        }
    }
}