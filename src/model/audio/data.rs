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
    pub frame_rate: SamplesPerFrame,
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
    pub frame_rate: Option<SamplesPerFrame>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub sample_rate: Option<u64>,
    pub segment_duration: Option<ScaledValue>,
    pub transmission: TrackTransmission,
}

impl AudioTrackData {
    pub fn with_default_codecs(mut self, codecs: &Option<String>) -> Self {
        if self.codecs.is_none() {
            self.codecs = codecs.clone()
        }
        self
    }

    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone()
        }
        self
    }

    pub fn with_default_initialization_pattern(
        mut self,
        initialization_pattern: &Option<String>,
    ) -> Self {
        if self.initialization_pattern.is_none() {
            self.initialization_pattern = initialization_pattern.clone()
        }
        self
    }

    pub fn with_default_frame_rate(mut self, frame_rate: SamplesPerFrame) -> Self {
        if self.frame_rate.is_none() {
            self.frame_rate = Some(frame_rate)
        }
        self
    }

    pub fn with_default_media_time_offset(mut self, media_time_offset: ScaledValue) -> Self {
        if self.media_time_offset.is_none() {
            self.media_time_offset = Some(media_time_offset)
        }
        self
    }

    pub fn with_default_sample_rate(mut self, sample_rate: Option<u64>) -> Self {
        if self.sample_rate.is_none() {
            self.sample_rate = sample_rate
        }
        self
    }
}
