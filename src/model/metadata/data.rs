use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::{
    normalize_tracks, Language, MetadataSwitchingSet, MetadataTrack, Number, ScaledDuration,
    ScaledValue, SegmentId, Segments,
};

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetData {
    pub id: String,
    pub mime_type: String,
    pub tracks: Vec<MetadataTrackData>,
    pub scheme_id: String,
    pub align_id: Option<String>,
    pub base_url: RelativeUrl,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub language: Option<Language>,
    pub media_time_offset: Option<ScaledValue>,
}

impl MetadataSwitchingSetData {
    pub fn new(input: MetadataSwitchingSet, location: &Url) -> Self {
        Self {
            id: input.id,
            mime_type: input.mime_type,
            tracks: input
                .tracks
                .into_iter()
                .map(|track| MetadataTrackData::new(track, location))
                .collect(),
            scheme_id: input.scheme_id,
            align_id: input.align_id,
            base_url: RelativeUrl::None,
            continuation_pattern: None,
            label: input.label,
            language: input.language,
            media_time_offset: None,
        }
    }

    pub fn normalize(&mut self) {
        normalize_tracks!(self, continuation_pattern, media_time_offset);
    }
}

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTrackData {
    pub id: String,
    pub segments: Segments,
    #[serde(rename = "activeSegment")]
    pub active_segment_id: Option<SegmentId>,
    pub average_bandwidth: Option<Number>,
    pub bandwidth: Option<Number>,
    pub base_url: RelativeUrl,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledDuration>,
}

impl MetadataTrackData {
    pub fn new(input: MetadataTrack, location: &Url) -> Self {
        Self {
            id: input.id().to_owned(),
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            average_bandwidth: input.average_bandwidth,
            bandwidth: input.bandwidth,
            base_url: RelativeUrl::None,
            continuation_pattern: Some(input.continuation_pattern.make_relative(location)),
            label: input.label,
            media_time_offset: Some(input.media_time_offset),
            segment_duration: input.segment_duration,
        }
    }
}

impl MetadataTrackData {
    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone();
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
