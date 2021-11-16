use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;
use crate::model::presentation::data::PresentationData;

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
