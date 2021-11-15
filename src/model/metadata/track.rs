use url::Url;

use crate::model::track::validate_segments;
use crate::util::{Entity, RelativeUrl};
use crate::*;

#[derive(Debug, Clone)]
pub struct MetadataTrack {
    uid: TrackUid,
    segments: Segments,
    active_segment_id: Option<SegmentId>,
    average_bandwidth: Option<Number>,
    bandwidth: Option<Number>,
    continuation_pattern: ContinuationPattern,
    label: Option<String>,
    media_time_offset: ScaledValue,
    segment_duration: Option<ScaledValue>,
}

impl Entity for MetadataTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for MetadataTrack {
    const TRACK_TYPE: TrackType = TrackType::Metadata;

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
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_url: &Url,
        data: MetadataTrackData,
        default_continuation_pattern: Option<&str>,
        default_media_time_offset: ScaledValue,
    ) -> Result<Self> {
        let MetadataTrackData {
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
        } = data;
        let base_url = base_url.resolve(switching_set_url)?;
        validate_segments(&id, segment_duration, &segments)?;
        default!(
            id,
            continuation_pattern,
            default_continuation_pattern,
            Error::MissingContinuationPattern
        );
        Ok(MetadataTrack {
            bandwidth,
            uid: TrackUid::new(presentation_id, Self::TRACK_TYPE, switching_set_id, id),
            segments,
            active_segment_id,
            average_bandwidth,
            continuation_pattern: ContinuationPattern::new(base_url, continuation_pattern)?,
            label,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            segment_duration,
        })
    }
}
