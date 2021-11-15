use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSetData {
    pub id: String,
    pub language: Language,
    pub tracks: Vec<AudioTrackData>,
    pub align_id: Option<String>,
    pub base_url: Option<String>,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    #[serde(default)]
    pub frame_rate: FrameRate,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    #[serde(default)]
    pub media_time_offset: ScaledValue,
    #[serde(default)]
    pub mime_type: AudioMimeType,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<u64>,
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackData {
    pub bandwidth: Number,
    pub id: String,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    pub base_url: Option<String>,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<FrameRate>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<u64>,
    pub segment_duration: Option<ScaledValue>,
    pub transmission: TrackTransmission,
}
