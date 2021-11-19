use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{
    DateTime, ManifestVersion, MulticastManifest, Number, PresentationData, UnicastManifest,
    UnicastStreamType,
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
    pub stream_type: UnicastStreamType,
    pub content_base_url: Option<String>,
}

//TODO make sure manifest location url is removed when serializing
impl From<UnicastManifest> for ManifestData {
    fn from(input: UnicastManifest) -> Self {
        Self {
            creation_date: input.creation_date,
            fallback_poll_rate: input.fallback_poll_rate,
            manifest_version: ManifestVersion::V1_0_0,
            presentations: input
                .presentations
                .into_iter()
                .map(PresentationData::from)
                .collect(),
            stream_type: input.stream_type,
            //TODO normalize all Urls, now the full URL is stored in all initialization-/continuation-patterns
            // same goes for switching set data defaults
            content_base_url: None,
        }
    }
}

impl From<MulticastManifest> for ManifestData {
    fn from(input: MulticastManifest) -> Self {
        Self {
            creation_date: input.creation_date,
            fallback_poll_rate: input.fallback_poll_rate,
            manifest_version: ManifestVersion::V1_0_0,
            presentations: input
                .presentations
                .into_iter()
                .map(PresentationData::from)
                .collect(),
            stream_type: input.stream_type.into(),
            content_base_url: None,
        }
    }
}
