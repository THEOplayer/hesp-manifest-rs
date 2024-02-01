use serde::Deserialize;

use crate::data::v2_0_0;
use crate::util::{UInt, Uri};
use crate::{Language, ScaledDuration, ScaledValue, SegmentId, Segments};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataSwitchingSetData {
    pub id: String,
    pub mime_type: String,
    pub tracks: Vec<MetadataTrackData>,
    pub scheme_id: String,
    pub align_id: Option<String>,
    pub base_url: Option<Uri>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub language: Option<Language>,
    pub media_time_offset: Option<ScaledValue>,
}

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTrackData {
    pub id: String,
    pub segments: Segments,
    pub active_segment: Option<SegmentId>,
    pub average_bandwidth: Option<UInt>,
    pub bandwidth: Option<UInt>,
    pub base_url: Option<Uri>,
    pub codecs: Option<String>,
    pub continuation_pattern: Option<String>,
    pub label: Option<String>,
    pub media_time_offset: Option<ScaledValue>,
    pub segment_duration: Option<ScaledDuration>,
}

impl From<MetadataSwitchingSetData> for v2_0_0::MetadataSwitchingSetData {
    fn from(input: MetadataSwitchingSetData) -> Self {
        Self {
            id: input.id,
            mime_type: input.mime_type,
            tracks: input
                .tracks
                .into_iter()
                .map(v2_0_0::MetadataTrackData::from)
                .collect(),
            scheme_id: input.scheme_id,
            align_id: input.align_id,
            base_url: input.base_url,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            label: input.label,
            language: input.language,
            media_time_offset: input.media_time_offset,
        }
    }
}

impl From<MetadataTrackData> for v2_0_0::MetadataTrackData {
    fn from(input: MetadataTrackData) -> Self {
        Self {
            id: input.id,
            segments: input.segments,
            //TODO should this be calculated? And if so, how?
            start_segment_id: SegmentId::default(),
            average_bandwidth: input.average_bandwidth,
            bandwidth: input.bandwidth,
            base_url: input.base_url,
            codecs: input.codecs,
            continuation_pattern: input.continuation_pattern,
            label: input.label,
            media_time_offset: input.media_time_offset,
            segment_duration: input.segment_duration,
        }
    }
}
