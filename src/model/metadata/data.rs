use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetData {
    pub id: String,
    pub mime_type: String,
    pub tracks: Vec<MetadataTrackData>,
    pub scheme_id: String,
    pub align_id: Option<String>,
    pub base_url: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub language: Option<Language>,
    #[serde(default)]
    pub media_time_offset: ScaledValue,
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTrackData {
    pub id: String,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub average_bandwidth: Option<Number>,
    pub bandwidth: Option<Number>,
    pub base_url: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledValue>,
}
