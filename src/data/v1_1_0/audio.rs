use serde::Deserialize;

use crate::data::v2_0_0;
use crate::util::{UInt, Uri};
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
    pub base_url: Option<Uri>,
    pub channels: Option<UInt>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<AudioMimeType>,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<UInt>,
}

impl From<AudioSwitchingSetData> for v2_0_0::AudioSwitchingSetData {
    fn from(input: AudioSwitchingSetData) -> Self {
        Self {
            id: input.id,
            language: input.language,
            tracks: input
                .tracks
                .into_iter()
                .map(v2_0_0::AudioTrackData::from)
                .collect(),
            align_id: input.align_id,
            base_url: input.base_url,
            channels: input.channels,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            samples_per_frame: input.samples_per_frame,
            initialization_pattern: input.initialization_pattern,
            label: input.label,
            media_time_offset: input.media_time_offset,
            mime_type: input.mime_type,
            protection: input.protection,
            sample_rate: input.sample_rate,
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
    pub base_url: Option<Uri>,
    pub channels: Option<UInt>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<UInt>,
    pub samples_per_frame: Option<SamplesPerFrame>,
    pub segment_duration: Option<ScaledDuration>,
}

impl From<AudioTrackData> for v2_0_0::AudioTrackData {
    fn from(input: AudioTrackData) -> Self {
        #[allow(deprecated)]
        Self {
            id: input.id,
            bandwidth: input.bandwidth,
            segments: input.segments,
            start_segment_id: SegmentId::default(),
            active_segment_id: input.active_segment,
            start_sequence_number: UInt::default(),
            active_sequence_number: input.active_sequence_number,
            average_bandwidth: input.average_bandwidth,
            base_url: input.base_url,
            channels: input.channels,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            samples_per_frame: input.samples_per_frame,
            label: input.label,
            initialization_pattern: input.initialization_pattern,
            media_time_offset: input.media_time_offset,
            sample_rate: input.sample_rate,
            segment_duration: input.segment_duration,
        }
    }
}
