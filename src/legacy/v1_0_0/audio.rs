use serde::Deserialize;

use crate::util::{RelativeUrl, UInt};
use crate::{
    AudioMimeType, Language, SamplesPerFrame, ScaledDuration, ScaledValue, SegmentId, Segments,
    SwitchingSetProtection,
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

impl From<AudioSwitchingSetData> for crate::AudioSwitchingSetData {
    fn from(input: AudioSwitchingSetData) -> Self {
        Self {
            id: input.id,
            language: input.language,
            tracks: input
                .tracks
                .into_iter()
                .map(crate::AudioTrackData::from)
                .collect(),
            align_id: input.align_id,
            base_url: input.base_url,
            channels: input.channels.map(UInt::from),
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            samples_per_frame: input.frame_rate,
            initialization_pattern: input.initialization_pattern,
            label: input.label,
            media_time_offset: input.media_time_offset,
            mime_type: input.mime_type,
            protection: input.protection,
            sample_rate: input.sample_rate.map(UInt::from),
        }
    }
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackData {
    pub id: String,
    pub bandwidth: UInt,
    pub segments: Segments,
    pub active_segment: Option<SegmentId>,
    pub active_sequence_number: Option<UInt>,
    pub average_bandwidth: Option<UInt>,
    pub base_url: RelativeUrl,
    pub channels: Option<UInt>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<UInt>,
    pub segment_duration: Option<ScaledDuration>,
}

impl From<AudioTrackData> for crate::AudioTrackData {
    fn from(input: AudioTrackData) -> Self {
        Self {
            id: input.id,
            bandwidth: input.bandwidth,
            segments: input.segments,
            active_segment_id: input.active_segment,
            active_sequence_number: input.active_sequence_number,
            average_bandwidth: input.average_bandwidth,
            base_url: input.base_url,
            channels: input.channels,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            samples_per_frame: input.frame_rate,
            label: input.label,
            initialization_pattern: input.initialization_pattern,
            media_time_offset: input.media_time_offset,
            sample_rate: input.sample_rate,
            segment_duration: input.segment_duration,
            toi_limits: None,
        }
    }
}
