use serde::Deserialize;

use crate::util::RelativeUrl;
use crate::{
    AudioMimeType, Language, Number, SamplesPerFrame, ScaledDuration, ScaledValue, SegmentId,
    Segments, SwitchingSetProtection, TransferObjectIdentifierLimits,
};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioSwitchingSetData {
    pub id: String,
    pub language: Language,
    pub tracks: Vec<AudioTrackData>,
    pub align_id: Option<String>,
    pub base_url: RelativeUrl,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<SamplesPerFrame>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<AudioMimeType>,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<u64>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackData {
    pub id: String,
    pub bandwidth: Number,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    pub base_url: RelativeUrl,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<u64>,
    pub segment_duration: Option<ScaledDuration>,
    pub toi_limits: Option<TransferObjectIdentifierLimits>,
}
