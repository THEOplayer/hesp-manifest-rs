use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetData {
    id: String,
    mime_type: String,
    tracks: Vec<MetadataTrackData>,
    scheme_id: String,
    align_id: Option<String>,
    base_url: Option<String>,
    continuation_pattern: Option<String>,
    label: Option<String>,
    language: Option<Language>,
    #[serde(default)]
    media_time_offset: ScaledValue,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTrackData {
    id: String,
    segments: Segments,
    #[serde(rename = "activeSegment")]
    active_segment_id: Option<SegmentId>,
    average_bandwidth: Option<Number>,
    bandwidth: Option<Number>,
    base_url: Option<String>,
    continuation_pattern: Option<String>,
    label: Option<String>,
    media_time_offset: Option<ScaledValue>,
    segment_duration: Option<ScaledValue>,
}
