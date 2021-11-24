use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::{
    Number, Resolution, ScaledDuration, ScaledValue, SegmentId, Segments, SwitchingSetProtection,
    TrackTransmission, VideoMimeType, VideoSwitchingSet, VideoTrack,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
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

impl VideoSwitchingSetData {
    pub fn new(input: VideoSwitchingSet, location: &Url) -> Self {
        Self {
            id: input.id,
            tracks: input
                .tracks
                .into_iter()
                .map(|track| VideoTrackData::new(track, location))
                .collect(),
            align_id: input.align_id,
            base_url: RelativeUrl::None,
            codecs: None,
            continuation_pattern: None,
            frame_rate: None,
            initialization_pattern: None,
            label: input.label,
            media_time_offset: None,
            mime_type: Some(input.mime_type),
            protection: input.protection,
        }
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    pub id: String,
    pub bandwidth: Number,
    pub resolution: Resolution,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
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
    pub transmission: TrackTransmission,
}

impl VideoTrackData {
    pub fn new(input: VideoTrack, location: &Url) -> Self {
        let id = input.id().to_owned();
        Self {
            id,
            bandwidth: input.bandwidth,
            resolution: input.resolution,
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            active_sequence_number: input.active_sequence_number,
            average_bandwidth: input.average_bandwidth,
            base_url: RelativeUrl::None,
            codecs: Some(input.codecs),
            continuation_pattern: Some(input.continuation_pattern.make_relative(location)),
            frame_rate: Some(input.frame_rate),
            label: input.label,
            initialization_pattern: Some(input.initialization_pattern.make_relative(location)),
            media_time_offset: Some(input.media_time_offset),
            segment_duration: input.segment_duration,
            transmission: input.transmission,
        }
    }
}

impl VideoTrackData {
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

    pub const fn with_default_frame_rate(mut self, frame_rate: Option<ScaledValue>) -> Self {
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
}
