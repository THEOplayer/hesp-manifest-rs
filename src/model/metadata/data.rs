use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Clone, Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetData {
    pub id: String,
    pub mime_type: String,
    pub tracks: Vec<MetadataTrackData>,
    pub scheme_id: String,
    pub align_id: Option<String>,
    pub base_url: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub language: Option<Language>,
    #[serde(default)]
    pub media_time_offset: ScaledValue,
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
    pub base_url: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledValue>,
}

impl MetadataTrackData {
    pub fn with_default_continuation_pattern(
        mut self,
        continuation_pattern: &Option<String>,
    ) -> Self {
        if self.continuation_pattern.is_none() {
            self.continuation_pattern = continuation_pattern.clone()
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
