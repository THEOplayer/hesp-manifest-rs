use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoSwitchingSetData {
    id: String,
    tracks: Vec<VideoTrackData>,
    align_id: Option<String>,
    base_url: Option<String>,
    codecs: Option<String>,
    continuation_pattern: Option<String>,
    frame_rate: Option<ScaledValue>,
    initialization_pattern: Option<String>,
    label: Option<String>,
    #[serde(default)]
    media_time_offset: ScaledValue,
    #[serde(default)]
    mime_type: VideoMimeType,
    protection: Option<SwitchingSetProtection>,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    bandwidth: Number,
    id: String,
    resolution: Resolution,
    segments: Segments,
    #[serde(rename = "activeSegment")]
    active_segment_id: Option<SegmentId>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    base_url: Option<String>,
    codecs: Option<String>,
    continuation_pattern: Option<String>,
    frame_rate: Option<ScaledValue>,
    label: Option<String>,
    initialization_pattern: Option<String>,
    media_time_offset: Option<ScaledValue>,
    segment_duration: Option<ScaledValue>,
    transmission: TrackTransmission,
}
