use crate::data::MetadataTrackData;
use crate::util::Entity;
use crate::{
    Address, ContinuationPattern, Error, MediaType, Result, ScaledDuration, ScaledValue, Segment,
    SegmentId, Segments, Track, TrackUid,
};

#[derive(Debug, Clone)]
pub struct MetadataTrack {
    uid: TrackUid,
    pub(crate) segments: Segments,
    pub(crate) start_segment_id: SegmentId,
    pub(crate) average_bandwidth: Option<u64>,
    pub(crate) bandwidth: Option<u64>,
    pub(crate) codecs: Option<String>,
    pub(crate) continuation_pattern: ContinuationPattern,
    pub(crate) label: Option<String>,
    pub(crate) media_time_offset: ScaledValue,
    pub(crate) mime_type: String,
    pub(crate) segment_duration: Option<ScaledDuration>,
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
    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn segments(&self) -> &[Segment] {
        &self.segments
    }

    fn start_segment_id(&self) -> SegmentId {
        self.start_segment_id
    }

    fn active_segment_id(&self) -> Option<SegmentId> {
        None
    }

    fn segment_duration(&self) -> Option<ScaledDuration> {
        self.segment_duration
    }
    fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }
    fn bandwidth(&self) -> Option<u64> {
        self.bandwidth
    }
    fn continuation_pattern(&self) -> &ContinuationPattern {
        &self.continuation_pattern
    }

    fn continuation_pattern_mut(&mut self) -> &mut ContinuationPattern {
        &mut self.continuation_pattern
    }

    fn media_type(&self) -> MediaType {
        Self::MEDIA_TYPE
    }

    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }
}

impl MetadataTrack {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_address: &Address,
        mime_type: String,
        data: MetadataTrackData,
    ) -> Result<Self> {
        let id = data.id;
        let address = switching_set_address.join(data.base_url)?;
        let continuation_pattern = data
            .continuation_pattern
            .ok_or_else(|| Error::MissingContinuationPattern(id.clone()))?;
        if data.segment_duration.is_none() {
            data.segments.ensure_time_bounds_defined(&id)?;
        }
        Ok(Self {
            bandwidth: data.bandwidth.map(u64::from),
            uid: TrackUid::new(presentation_id, Self::MEDIA_TYPE, switching_set_id, id),
            segments: data.segments,
            start_segment_id: data.start_segment_id,
            average_bandwidth: data.average_bandwidth.map(u64::from),
            continuation_pattern: ContinuationPattern::new(address, continuation_pattern)?,
            label: data.label,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            mime_type,
            segment_duration: data.segment_duration,
            codecs: data.codecs,
        })
    }
}
