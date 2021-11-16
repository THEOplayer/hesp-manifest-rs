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

impl VideoTrackData {
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

    pub fn with_default_frame_rate(mut self, frame_rate: Option<ScaledValue>) -> Self {
        if self.frame_rate.is_none() {
            self.frame_rate = frame_rate
        }
        self
    }

    pub fn with_default_media_time_offset(mut self, media_time_offset: ScaledValue) -> Self {
        if self.media_time_offset.is_none() {
            self.media_time_offset = Some(media_time_offset)
        }
        self
    }
}
