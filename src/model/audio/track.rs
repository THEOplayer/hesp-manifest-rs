use crate::util::Entity;
use crate::{
    Address, AudioMimeType, AudioTrackData, ContinuationPattern, Error, FrameRate, Initialization,
    InitializationPattern, MediaType, Result, SamplesPerFrame, ScaledDuration, ScaledValue,
    Segment, SegmentId, Segments, Track, TrackTransmission, TrackUid, ValidateTrack,
};

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub(super) bandwidth: u64,
    uid: TrackUid,
    pub(super) segments: Segments,
    pub(super) active_segment_id: Option<SegmentId>,
    pub(super) active_sequence_number: Option<u64>,
    pub(super) average_bandwidth: Option<u64>,
    pub(super) channels: Option<u64>,
    pub(super) codecs: String,
    pub(super) continuation_pattern: ContinuationPattern,
    pub(super) samples_per_frame: SamplesPerFrame,
    pub(super) label: Option<String>,
    pub(super) initialization_pattern: InitializationPattern,
    pub(super) media_time_offset: ScaledValue,
    pub(super) mime_type: AudioMimeType,
    pub(super) sample_rate: u64,
    pub(super) segment_duration: Option<ScaledDuration>,
    pub(crate) transmission: TrackTransmission,
}

impl AudioTrack {
    const MEDIA_TYPE: MediaType = MediaType::Audio;
}

impl Entity for AudioTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for AudioTrack {
    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn bandwidth(&self) -> Option<u64> {
        Some(self.bandwidth)
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

    fn continuation_pattern_mut(&mut self) -> &mut ContinuationPattern {
        &mut self.continuation_pattern
    }

    fn average_bandwidth(&self) -> Option<u64> {
        self.average_bandwidth
    }

    fn media_type(&self) -> MediaType {
        Self::MEDIA_TYPE
    }

    fn mime_type(&self) -> &str {
        self.mime_type.as_ref()
    }

    fn transmission(&self) -> &TrackTransmission {
        &self.transmission
    }
}

impl ValidateTrack for AudioTrack {
    fn validate_active(&self) -> Result<()> {
        Initialization::validate_active(self)
    }
}

impl Initialization for AudioTrack {
    fn initialization_pattern(&self) -> &InitializationPattern {
        &self.initialization_pattern
    }

    fn initialization_pattern_mut(&mut self) -> &mut InitializationPattern {
        &mut self.initialization_pattern
    }

    fn active_sequence_number(&self) -> Option<u64> {
        self.active_sequence_number
    }

    fn frame_rate(&self) -> FrameRate {
        FrameRate::new(self.sample_rate, self.samples_per_frame.into())
    }
}

impl AudioTrack {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_address: &Address,
        mime_type: AudioMimeType,
        data: AudioTrackData,
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
        let sample_rate = data
            .sample_rate
            .ok_or_else(|| Error::MissingSampleRate(id.clone()))?
            .into();
        if data.segment_duration.is_none() {
            data.segments.ensure_time_bounds_defined(&id)?;
        }
        Ok(Self {
            bandwidth: data.bandwidth.into(),
            uid: TrackUid::new(presentation_id, Self::MEDIA_TYPE, switching_set_id, id),
            segments: data.segments,
            active_segment_id: data.active_segment_id,
            active_sequence_number: data.active_sequence_number.map(u64::from),
            average_bandwidth: data.average_bandwidth.map(u64::from),
            channels: data.channels.map(u64::from),
            codecs,
            continuation_pattern: ContinuationPattern::new(address.clone(), continuation_pattern)?,
            samples_per_frame: data.samples_per_frame.unwrap_or_default(),
            label: data.label,
            initialization_pattern: InitializationPattern::new(address, initialization_pattern)?,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            mime_type,
            sample_rate,
            segment_duration: data.segment_duration,
            transmission: data.toi_limits.into(),
        })
    }
}
