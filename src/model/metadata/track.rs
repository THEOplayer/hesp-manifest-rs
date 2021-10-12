use serde::{self, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::track::validate_segments;
use crate::*;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct MetadataTrack {
    id: String,
    segments: Segments,
    #[serde(rename = "activeSegment")]
    active_segment_id: Option<SegmentId>,
    average_bandwidth: Option<Number>,
    bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    continuation_pattern: ContinuationPattern,
    label: Option<String>,
    media_time_offset: ScaledValue,
    segment_duration: Option<ScaledValue>,
}

impl Entity for MetadataTrack {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}

impl Track for MetadataTrack {
    fn active_segment(&self) -> Option<&Segment> {
        match self.active_segment_id {
            Some(id) => self.segment(id),
            None => None,
        }
    }
    fn segment_duration(&self) -> Option<ScaledValue> {
        self.segment_duration
    }
    fn segments(&self) -> &[Segment] {
        &self.segments
    }
    fn base_url(&self) -> &Option<RelativeBaseUrl> {
        &self.base_url
    }
    fn base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl> {
        &mut self.base_url
    }
    fn continuation_pattern(&self) -> &ContinuationPattern {
        &self.continuation_pattern
    }
    fn set_continuation_pattern(&mut self, pattern: ContinuationPattern) {
        self.continuation_pattern = pattern;
    }
    fn average_bandwidth(&self) -> Option<f64> {
        self.average_bandwidth.as_ref().and_then(Number::as_f64)
    }
}

impl MetadataTrack {
    pub(super) fn new(
        def: MetadataTrackDef,
        default_continuation_pattern: Option<&ContinuationPattern>,
        default_media_time_offset: ScaledValue,
    ) -> Result<Self> {
        let MetadataTrackDef {
            bandwidth,
            id,
            segments,
            active_segment_id,
            average_bandwidth,
            base_url,
            continuation_pattern,
            label,
            media_time_offset,
            segment_duration,
        } = def;
        validate_segments(&id, segment_duration, &segments)?;
        default!(
            id,
            continuation_pattern,
            default_continuation_pattern,
            Error::MissingContinuationPattern
        );
        Ok(MetadataTrack {
            bandwidth,
            id,
            segments,
            active_segment_id,
            average_bandwidth,
            base_url,
            continuation_pattern,
            label,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            segment_duration,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct MetadataTrackDef {
    id: String,
    segments: Segments,
    #[serde(rename = "activeSegment")]
    active_segment_id: Option<SegmentId>,
    average_bandwidth: Option<Number>,
    bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    continuation_pattern: Option<ContinuationPattern>,
    label: Option<String>,
    media_time_offset: Option<ScaledValue>,
    segment_duration: Option<ScaledValue>,
}

impl Entity for MetadataTrackDef {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}
