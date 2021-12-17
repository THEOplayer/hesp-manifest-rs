use url::Url;

use crate::util::Entity;
use crate::{
    ContinuationPattern, Error, MediaType, MetadataTrackData, Result, ScaledDuration, ScaledValue,
    Segment, SegmentId, Segments, Track, TrackTransmission, TrackUid,
};

#[derive(Debug, Clone)]
pub struct MetadataTrack {
    uid: TrackUid,
    pub(super) segments: Segments,
    pub(super) active_segment_id: Option<SegmentId>,
    pub(super) average_bandwidth: Option<u64>,
    pub(super) bandwidth: Option<u64>,
    pub(super) codecs: Option<String>,
    pub(super) continuation_pattern: ContinuationPattern,
    pub(super) label: Option<String>,
    pub(super) media_time_offset: ScaledValue,
    pub(super) segment_duration: Option<ScaledDuration>,
}

impl MetadataTrack {
    const MEDIA_TYPE: MediaType = MediaType::Metadata;
}

impl Entity for MetadataTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for MetadataTrack {
    fn media_type(&self) -> MediaType {
        Self::MEDIA_TYPE
    }

    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn bandwidth(&self) -> Option<u64> {
        self.bandwidth
    }

    fn active_segment(&self) -> Option<&Segment> {
        match self.active_segment_id {
            Some(id) => self.segment(id),
            None => None,
        }
    }
    fn segment_duration(&self) -> Option<ScaledDuration> {
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
    fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    fn transmission(&self) -> &TrackTransmission {
        &TrackTransmission::Unicast
    }

    fn validate_active(&self) -> Result<()> {
        Ok(())
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
            bandwidth: data.bandwidth.map(u64::from),
            uid: TrackUid::new(presentation_id, Self::MEDIA_TYPE, switching_set_id, id),
            segments: data.segments,
            active_segment_id: data.active_segment_id,
            average_bandwidth: data.average_bandwidth.map(u64::from),
            continuation_pattern: ContinuationPattern::new(&base_url, continuation_pattern)?,
            label: data.label,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            segment_duration: data.segment_duration,
            codecs: data.codecs,
        })
    }
}
