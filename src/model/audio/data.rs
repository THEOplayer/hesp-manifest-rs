use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::Entity;
use crate::{
    AudioMimeType, AudioSwitchingSet, AudioTrack, Language, Number, SamplesPerFrame,
    ScaledDuration, ScaledValue, SegmentId, Segments, SwitchingSetProtection, TrackTransmission,
};

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
    pub frame_rate: Option<SamplesPerFrame>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<AudioMimeType>,
    pub protection: Option<SwitchingSetProtection>,
    pub sample_rate: Option<u64>,
}

impl From<AudioSwitchingSet> for AudioSwitchingSetData {
    fn from(input: AudioSwitchingSet) -> Self {
        Self {
            id: input.id,
            language: input.language,
            tracks: input.tracks.into_iter().map(From::from).collect(),
            align_id: input.align_id,
            base_url: None,
            channels: input.channels,
            codecs: None,
            continuation_pattern: None,
            frame_rate: None,
            initialization_pattern: None,
            label: input.label,
            media_time_offset: None,
            mime_type: None,
            protection: input.protection,
            sample_rate: None,
        }
    }
}

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrackData {
    pub id: String,
    pub bandwidth: Number,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<u64>,
    pub average_bandwidth: Option<Number>,
    pub base_url: Option<String>,
    pub channels: Option<u64>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<u64>,
    pub segment_duration: Option<ScaledDuration>,
    pub transmission: TrackTransmission,
}

impl From<AudioTrack> for AudioTrackData {
    fn from(input: AudioTrack) -> Self {
        let id = input.id().to_owned();
        Self {
            id,
            bandwidth: input.bandwidth,
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            active_sequence_number: input.active_sequence_number,
            average_bandwidth: input.average_bandwidth,
            base_url: None,
            channels: input.channels,
            codecs: Some(input.codecs),
            continuation_pattern: Some(input.continuation_pattern.to_string()),
            frame_rate: Some(input.frame_rate),
            label: input.label,
            initialization_pattern: Some(input.initialization_pattern.to_string()),
            media_time_offset: Some(input.media_time_offset),
            sample_rate: Some(input.sample_rate),
            segment_duration: input.segment_duration,
            transmission: input.transmission,
        }
    }
}

impl AudioTrackData {
    pub fn with_default_codecs(mut self, codecs: &Option<String>) -> Self {
        if self.codecs.is_none() {
            self.codecs = codecs.clone();
        }
        self
    }

    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone();
        }
        self
    }

    pub fn with_default_initialization_pattern(
        mut self,
        initialization_pattern: &Option<String>,
    ) -> Self {
        if self.initialization_pattern.is_none() {
            self.initialization_pattern = initialization_pattern.clone();
        }
        self
    }

    pub const fn with_default_frame_rate(mut self, frame_rate: Option<SamplesPerFrame>) -> Self {
        if self.frame_rate.is_none() {
            self.frame_rate = frame_rate;
        }
        self
    }

    pub const fn with_default_media_time_offset(
        mut self,
        media_time_offset: Option<ScaledValue>,
    ) -> Self {
        if self.media_time_offset.is_none() {
            self.media_time_offset = media_time_offset;
        }
        self
    }

    pub const fn with_default_sample_rate(mut self, sample_rate: Option<u64>) -> Self {
        if self.sample_rate.is_none() {
            self.sample_rate = sample_rate;
        }
        self
    }
}
