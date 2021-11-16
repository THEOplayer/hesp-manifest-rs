use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoSwitchingSetData {
    pub id: String,
    pub tracks: Vec<VideoTrackData>,
    pub align_id: Option<String>,
    pub base_url: Option<String>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<ScaledValue>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    #[serde(default)]
    pub media_time_offset: ScaledValue,
    #[serde(default)]
    pub mime_type: VideoMimeType,
    pub protection: Option<SwitchingSetProtection>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    pub bandwidth: Number,
    pub id: String,
    pub resolution: Resolution,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    pub base_url: Option<String>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<ScaledValue>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledValue>,
    pub transmission: TrackTransmission,
}
