use crate::util::Entity;
use crate::{
    Address, ContinuationPattern, Error, FrameRate, Initialization, InitializationPattern,
    MediaType, Resolution, Result, ScaledDuration, ScaledValue, Segment, SegmentId, Segments,
    Track, TrackUid, VideoMimeType, VideoTrackData,
};

#[derive(Debug, Clone)]
pub struct VideoTrack {
    uid: TrackUid,
    pub(super) bandwidth: u64,
    pub(super) resolution: Resolution,
    pub(super) segments: Segments,
    pub(super) start_segment_id: SegmentId,
    pub(super) start_sequence_number: u64,
    pub(super) average_bandwidth: Option<u64>,
    pub(super) codecs: String,
    pub(super) continuation_pattern: ContinuationPattern,
    pub(super) frame_rate: FrameRate,
    pub(super) label: Option<String>,
    pub(super) initialization_pattern: InitializationPattern,
    pub(super) media_time_offset: ScaledValue,
    pub(super) mime_type: VideoMimeType,
    pub(super) segment_duration: Option<ScaledDuration>,
}

impl VideoTrack {
    const MEDIA_TYPE: MediaType = MediaType::Video;
}

impl Entity for VideoTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for VideoTrack {
    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn segments(&self) -> &[Segment] {
        &self.segments
    }

    fn start_segment_id(&self) -> SegmentId {
        self.start_segment_id
    }

    fn segment_duration(&self) -> Option<ScaledDuration> {
        self.segment_duration
    }

    fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    fn bandwidth(&self) -> Option<u64> {
        Some(self.bandwidth)
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

impl Initialization for VideoTrack {
    fn initialization_pattern(&self) -> &InitializationPattern {
        &self.initialization_pattern
    }

    fn initialization_pattern_mut(&mut self) -> &mut InitializationPattern {
        &mut self.initialization_pattern
    }

    fn start_sequence_number(&self) -> u64 {
        self.start_sequence_number
    }

    fn frame_rate(&self) -> FrameRate {
        self.frame_rate
    }
}

impl VideoTrack {
    pub(super) fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_address: &Address,
        mime_type: VideoMimeType,
        data: VideoTrackData,
    ) -> Result<Self> {
        let id = data.id;
        let address = switching_set_address.join(data.base_url)?;
        let codecs = data
            .codecs
            .ok_or_else(|| Error::MissingCodecs(id.clone()))?;
        let continuation_pattern = data
            .continuation_pattern
            .ok_or_else(|| Error::MissingContinuationPattern(id.clone()))?;
        let initialization_pattern = data
            .initialization_pattern
            .ok_or_else(|| Error::MissingInitializationPattern(id.clone()))?;
        let frame_rate = data
            .frame_rate
            .ok_or_else(|| Error::MissingFrameRate(id.clone()))?;
        if data.segment_duration.is_none() {
            data.segments.ensure_time_bounds_defined(&id)?;
        }
        Ok(Self {
            bandwidth: data.bandwidth.into(),
            uid: TrackUid::new(presentation_id, Self::MEDIA_TYPE, switching_set_id, id),
            resolution: data.resolution,
            segments: data.segments,
            start_segment_id: data.start_segment_id,
            start_sequence_number: data.start_sequence_number.into(),
            average_bandwidth: data.average_bandwidth.map(u64::from),
            codecs,
            continuation_pattern: ContinuationPattern::new(address.clone(), continuation_pattern)?,
            frame_rate,
            label: data.label,
            initialization_pattern: InitializationPattern::new(address, initialization_pattern)?,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            mime_type,
            segment_duration: data.segment_duration,
        })
    }

    pub const fn start_sequence_number(&self) -> u64 {
        self.start_sequence_number
    }
}
