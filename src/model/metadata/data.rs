use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::{Entity, RelativeUrl, UInt};
use crate::{
    normalize_tracks, Language, MetadataSwitchingSet, MetadataTrack, ScaledDuration, ScaledValue,
    SegmentId, Segments,
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
    #[serde(skip_serializing_if = "RelativeUrl::is_none")]
    pub base_url: RelativeUrl,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub language: Option<Language>,
    pub media_time_offset: Option<ScaledValue>,
}

impl From<MetadataSwitchingSet> for MetadataSwitchingSetData {
    fn from(input: MetadataSwitchingSet) -> Self {
        Self {
            id: input.id,
            mime_type: input.mime_type,
            tracks: input
                .tracks
                .into_iter()
                .map(MetadataTrackData::from)
                .collect(),
            scheme_id: input.scheme_id,
            align_id: input.align_id,
            base_url: RelativeUrl::None,
            codecs: None,
            continuation_pattern: None,
            label: input.label,
            language: input.language,
            media_time_offset: None,
        }
    }
}

impl MetadataSwitchingSetData {
    pub fn normalize(&mut self) {
        normalize_tracks!(self, codecs, continuation_pattern, media_time_offset);
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
    pub average_bandwidth: Option<UInt>,
    pub bandwidth: Option<UInt>,
    #[serde(skip_serializing_if = "RelativeUrl::is_none")]
    pub base_url: RelativeUrl,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledDuration>,
}

impl From<MetadataTrack> for MetadataTrackData {
    fn from(input: MetadataTrack) -> Self {
        Self {
            id: input.id().to_owned(),
            segments: input.segments,
            active_segment_id: input.active_segment_id,
            average_bandwidth: input.average_bandwidth.map(UInt::from),
            bandwidth: input.bandwidth.map(UInt::from),
            base_url: input.continuation_pattern.base_url().clone(),
            codecs: input.codecs,
            continuation_pattern: Some(input.continuation_pattern.into_pattern()),
            label: input.label,
            media_time_offset: Some(input.media_time_offset),
            segment_duration: input.segment_duration,
        }
    }
}

impl MetadataTrackData {
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
