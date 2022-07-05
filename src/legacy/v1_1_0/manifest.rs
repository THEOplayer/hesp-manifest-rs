use crate::legacy::{v1_0_0, v1_1_0};
use crate::{Error, Result};
use serde::Deserialize;

use crate::util::{Timestamp, UInt, Uri};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: Timestamp,
    pub fallback_poll_rate: UInt,
    pub presentations: Vec<v1_1_0::PresentationData>,
    #[serde(flatten)]
    pub stream_type: v1_0_0::StreamType,
    pub content_base_url: Option<Uri>,
}

impl TryFrom<ManifestData> for crate::ManifestData {
    type Error = Error;

    fn try_from(value: ManifestData) -> Result<Self> {
        Ok(Self {
            creation_date: value.creation_date,
            fallback_poll_rate: value.fallback_poll_rate,
            stream_type: value.stream_type.convert(&value.presentations)?,
            presentations: value
                .presentations
                .into_iter()
                .map(crate::PresentationData::from)
                .collect(),
            content_base_url: value.content_base_url,
            multicast_metadata: None,
        })
    }
}
