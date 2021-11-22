use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::{
    ContinuationPattern, Error, MediaType, MetadataTrackData, Number, Result, ScaledValue, Segment,
    SegmentId, Segments, Track, TrackTransmission, TrackUid,
};

#[derive(Debug, Clone)]
pub struct MetadataTrack {
    uid: TrackUid,
    pub(super) segments: Segments,
    pub(super) active_segment_id: Option<SegmentId>,
    pub(super) average_bandwidth: Option<Number>,
    pub(super) bandwidth: Option<Number>,
    pub(super) continuation_pattern: ContinuationPattern,
    pub(super) label: Option<String>,
    pub(super) media_time_offset: ScaledValue,
    pub(super) segment_duration: Option<ScaledValue>,
}

impl Entity for MetadataTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for MetadataTrack {
    const TRACK_TYPE: MediaType = MediaType::Metadata;

    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn bandwidth(&self) -> Option<f64> {
        self.bandwidth.as_ref().and_then(Number::as_f64)
    }

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

    fn transmission(&self) -> &TrackTransmission {
        todo!()
    }

    fn validate_active(&self) -> Result<()> {
        todo!()
    }
}

impl MetadataTrack {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_url: &Url,
        data: MetadataTrackData,
    ) -> Result<Self> {
        let id = data.id;
        let base_url = data.base_url.resolve(switching_set_url)?;
        let continuation_pattern = if let Some(continuation_pattern) = data.continuation_pattern {
            continuation_pattern
        } else {
            return Err(Error::MissingContinuationPattern(id));
        };
        Ok(Self {
            bandwidth: data.bandwidth,
            uid: TrackUid::new(presentation_id, Self::TRACK_TYPE, switching_set_id, id),
            segments: data.segments,
            active_segment_id: data.active_segment_id,
            average_bandwidth: data.average_bandwidth,
            continuation_pattern: ContinuationPattern::new(base_url, continuation_pattern)?,
            label: data.label,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            segment_duration: data.segment_duration,
        })
    }
}
