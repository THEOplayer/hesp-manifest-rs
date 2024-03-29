use crate::data::AudioTrackData;
use crate::util::Entity;
use crate::{
    Address, AudioMimeType, ContinuationPattern, Error, FrameRate, Initialization,
    InitializationPattern, MediaType, Result, SamplesPerFrame, ScaledDuration, ScaledValue,
    Segment, SegmentId, Segments, Track, TrackUid,
};

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub(crate) bandwidth: u64,
    pub(crate) uid: TrackUid,
    pub(crate) segments: Segments,
    pub(crate) start_segment_id: SegmentId,
    #[deprecated(note = "please use `start_segment_id` instead")]
    pub(crate) active_segment_id: Option<SegmentId>,
    pub(crate) start_sequence_number: u64,
    #[deprecated(note = "please use `start_sequence_number` instead")]
    pub(crate) active_sequence_number: Option<u64>,
    pub(crate) average_bandwidth: Option<u64>,
    pub(crate) channels: Option<u64>,
    pub(crate) codecs: String,
    pub(crate) continuation_pattern: ContinuationPattern,
    pub(crate) samples_per_frame: SamplesPerFrame,
    pub(crate) label: Option<String>,
    pub(crate) initialization_pattern: InitializationPattern,
    pub(crate) media_time_offset: ScaledValue,
    pub(crate) mime_type: AudioMimeType,
    pub(crate) sample_rate: u64,
    pub(crate) segment_duration: Option<ScaledDuration>,
}

impl AudioTrack {
    const MEDIA_TYPE: MediaType = MediaType::Audio;

    #[must_use]
    pub const fn start_sequence_number(&self) -> u64 {
        self.start_sequence_number
    }
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

    fn segments(&self) -> &[Segment] {
        &self.segments
    }

    fn start_segment_id(&self) -> SegmentId {
        self.start_segment_id
    }

    #[allow(deprecated)]
    fn active_segment_id(&self) -> Option<SegmentId> {
        self.active_segment_id
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

impl Initialization for AudioTrack {
    fn initialization_pattern(&self) -> &InitializationPattern {
        &self.initialization_pattern
    }

    fn initialization_pattern_mut(&mut self) -> &mut InitializationPattern {
        &mut self.initialization_pattern
    }

    fn start_sequence_number(&self) -> u64 {
        self.start_sequence_number
    }

    #[allow(deprecated)]
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
        #[allow(deprecated)]
        Ok(Self {
            bandwidth: data.bandwidth.into(),
            uid: TrackUid::new(presentation_id, Self::MEDIA_TYPE, switching_set_id, id),
            segments: data.segments,
            start_segment_id: data.start_segment_id,
            active_segment_id: data.active_segment_id,
            start_sequence_number: data.start_sequence_number.into(),
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
        })
    }
}
