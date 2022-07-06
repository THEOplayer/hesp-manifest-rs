use crate::util::{UInt, Uri};
use crate::{
    FrameRate, Resolution, ScaledDuration, ScaledValue, SegmentId, Segments,
    SwitchingSetProtection, TrackMulticastMetadata, VideoMimeType,
};
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
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

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrackData {
    pub id: String,
    pub bandwidth: UInt,
    pub resolution: Resolution,
    pub segments: Segments,
    pub active_segment: Option<SegmentId>,
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

impl From<VideoSwitchingSetData> for crate::VideoSwitchingSetData {
    fn from(input: VideoSwitchingSetData) -> Self {
        Self {
            id: input.id,
            tracks: input
                .tracks
                .into_iter()
                .map(crate::VideoTrackData::from)
                .collect(),
            align_id: input.align_id,
            base_url: input.base_url,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            frame_rate: input.frame_rate,
            initialization_pattern: input.initialization_pattern,
            label: input.label,
            media_time_offset: input.media_time_offset,
            mime_type: input.mime_type,
            protection: input.protection,
        }
    }
}

impl From<VideoTrackData> for crate::VideoTrackData {
    fn from(input: VideoTrackData) -> Self {
        Self {
            id: input.id,
            bandwidth: input.bandwidth,
            resolution: input.resolution,
            segments: input.segments,
            start_segment_id: SegmentId::default(), //TODO: calculate?
            start_sequence_number: UInt::default(), //TODO: calculate?
            average_bandwidth: input.average_bandwidth,
            base_url: input.base_url,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            frame_rate: input.frame_rate,
            label: input.label,
            initialization_pattern: input.initialization_pattern,
            media_time_offset: input.media_time_offset,
            segment_duration: input.segment_duration,
            multicast_metadata: input.multicast_metadata,
        }
    }
}
