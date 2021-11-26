use serde::Deserialize;

use crate::util::RelativeUrl;
use crate::{
    Number, Resolution, ScaledDuration, ScaledValue, SegmentId, Segments, SwitchingSetProtection,
    TransferObjectIdentifierLimits, VideoMimeType,
};

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoSwitchingSetData {
    pub id: String,
    pub tracks: Vec<VideoTrackData>,
    pub align_id: Option<String>,
    pub base_url: RelativeUrl,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<ScaledValue>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<VideoMimeType>,
    pub protection: Option<SwitchingSetProtection>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    pub id: String,
    pub bandwidth: Number,
    pub resolution: Resolution,
    pub segments: Segments,
    pub active_segment: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    pub base_url: RelativeUrl,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<ScaledValue>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledDuration>,
    pub toi_limits: Option<TransferObjectIdentifierLimits>,
}
