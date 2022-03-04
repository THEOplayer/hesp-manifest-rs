use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::{Entity, UInt, Uri};
use crate::{
    normalize_tracks, FrameRate, Resolution, ScaledDuration, ScaledValue, SegmentId, Segments,
    SwitchingSetProtection, TrackMulticastMetadata, VideoMimeType, VideoSwitchingSet, VideoTrack,
};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoSwitchingSetData {
    pub id: String,
    pub tracks: Vec<VideoTrackData>,
    pub align_id: Option<String>,
    pub base_url: Option<Uri>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<FrameRate>,
    pub initialization_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub mime_type: Option<VideoMimeType>,
    pub protection: Option<SwitchingSetProtection>,
}

impl From<VideoSwitchingSet> for VideoSwitchingSetData {
    fn from(input: VideoSwitchingSet) -> Self {
        Self {
            id: input.id,
            tracks: input.tracks.into_iter().map(VideoTrackData::from).collect(),
            align_id: input.align_id,
            base_url: None,
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

impl VideoSwitchingSetData {
    pub fn normalize(&mut self) {
        normalize_tracks!(
            self,
            codecs,
            continuation_pattern,
            frame_rate,
            initialization_pattern,
            media_time_offset
        );
    }
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    pub id: String,
    pub bandwidth: UInt,
    pub resolution: Resolution,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub active_sequence_number: Option<UInt>,
    pub average_bandwidth: Option<UInt>,
    pub base_url: Option<Uri>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub frame_rate: Option<FrameRate>,
    pub label: Option<String>,
    pub initialization_pattern: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledDuration>,
    pub multicast_metadata: Option<TrackMulticastMetadata>,
}

impl From<VideoTrack> for VideoTrackData {
    fn from(input: VideoTrack) -> Self {
        let id = input.id().to_owned();
        let (base_url, continuation_pattern, initialization_pattern) =
            if input.continuation_pattern.base_url() == input.initialization_pattern.base_url() {
                (
                    input.continuation_pattern.base_url().cloned(),
                    input.continuation_pattern.into_pattern(),
                    input.initialization_pattern.into_pattern(),
                )
            } else {
                (
                    None,
                    input.continuation_pattern.into_full_pattern(),
                    input.initialization_pattern.into_full_pattern(),
                )
            };
        Self {
            id,
            bandwidth: input.bandwidth.into(),
            resolution: input.resolution,
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            active_sequence_number: input.active_sequence_number.map(UInt::from),
            average_bandwidth: input.average_bandwidth.map(UInt::from),
            base_url,
            codecs: Some(input.codecs),
            continuation_pattern: Some(continuation_pattern),
            frame_rate: Some(input.frame_rate),
            label: input.label,
            initialization_pattern: Some(initialization_pattern),
            media_time_offset: Some(input.media_time_offset),
            segment_duration: input.segment_duration,
            multicast_metadata: input.transmission.into(),
        }
    }
}

impl VideoTrackData {
    #[must_use]
    pub fn with_default_codecs(mut self, codecs: &Option<String>) -> Self {
        if self.codecs.is_none() {
            self.codecs = codecs.clone();
        }
        self
    }

    #[must_use]
    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone();
        }
        self
    }

    #[must_use]
    pub fn with_default_initialization_pattern(
        mut self,
        initialization_pattern: &Option<String>,
    ) -> Self {
        if self.initialization_pattern.is_none() {
            self.initialization_pattern = initialization_pattern.clone();
        }
        self
    }

    #[must_use]
    pub const fn with_default_frame_rate(mut self, frame_rate: Option<FrameRate>) -> Self {
        if self.frame_rate.is_none() {
            self.frame_rate = frame_rate;
        }
        self
    }

    #[must_use]
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
