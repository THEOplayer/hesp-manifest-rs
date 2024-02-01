use serde::Deserialize;

use crate::data::{v1_1_0, v2_0_0};
use crate::util::{Timestamp, UInt, Uri};
use crate::{Error, Result};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ManifestData {
    pub creation_date: Timestamp,
    pub fallback_poll_rate: UInt,
    pub presentations: Vec<v1_1_0::PresentationData>,
    #[serde(flatten)]
    pub stream_type: v1_1_0::StreamType,
    pub content_base_url: Option<Uri>,
}

impl TryFrom<ManifestData> for v2_0_0::ManifestData {
    type Error = Error;

    fn try_from(value: ManifestData) -> Result<Self> {
        Ok(Self {
            creation_date: value.creation_date,
            fallback_poll_rate: value.fallback_poll_rate,
            stream_type: value.stream_type.convert(&value.presentations)?,
            presentations: value
                .presentations
                .into_iter()
                .map(v2_0_0::PresentationData::from)
                .collect(),
            content_base_url: value.content_base_url,
        })
    }
}
