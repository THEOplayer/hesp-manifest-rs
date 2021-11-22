use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::{
    AudioTrackData, ContinuationPattern, Error, Initialization, InitializationPattern, MediaType,
    Number, Result, SamplesPerFrame, ScaledValue, Segment, SegmentId, Segments, Track,
    TrackTransmission, TrackUid,
};

#[derive(Debug, Clone)]
pub struct AudioTrack {
    pub(super) bandwidth: Number,
    uid: TrackUid,
    pub(super) segments: Segments,
    pub(super) active_segment_id: Option<SegmentId>,
    pub(super) active_sequence_number: Option<u64>,
    pub(super) average_bandwidth: Option<Number>,
    pub(super) channels: Option<u64>,
    pub(super) codecs: String,
    pub(super) continuation_pattern: ContinuationPattern,
    pub(super) frame_rate: SamplesPerFrame,
    pub(super) label: Option<String>,
    pub(super) initialization_pattern: InitializationPattern,
    pub(super) media_time_offset: ScaledValue,
    pub(super) sample_rate: u64,
    pub(super) segment_duration: Option<ScaledValue>,
    pub(crate) transmission: TrackTransmission,
}

impl Entity for AudioTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for AudioTrack {
    const TRACK_TYPE: MediaType = MediaType::Audio;

    fn uid(&self) -> &TrackUid {
        &self.uid
    }

    fn bandwidth(&self) -> Option<f64> {
        Some(self.bandwidth.as_f64().unwrap())
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
        &self.transmission
    }

    fn validate_active(&self) -> Result<()> {
        Initialization::validate_active(self)
    }
}

impl Initialization for AudioTrack {
    fn initialization_pattern(&self) -> &InitializationPattern {
        &self.initialization_pattern
    }

    fn set_initialization_pattern(&mut self, pattern: InitializationPattern) {
        self.initialization_pattern = pattern;
    }

    fn active_sequence_number(&self) -> Option<u64> {
        self.active_sequence_number
    }
}

impl AudioTrack {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_url: &Url,
        data: AudioTrackData,
    ) -> Result<Self> {
        let id = data.id;
        let base_url = data.base_url.resolve(switching_set_url)?;
        let codecs = if let Some(codecs) = data.codecs {
            codecs
        } else {
            return Err(Error::MissingCodecs(id));
        };
        let continuation_pattern = if let Some(continuation_pattern) = data.continuation_pattern {
            continuation_pattern
        } else {
            return Err(Error::MissingContinuationPattern(id));
        };
        let initialization_pattern =
            if let Some(initialization_pattern) = data.initialization_pattern {
                initialization_pattern
            } else {
                return Err(Error::MissingInitializationPattern(id));
            };
        let sample_rate = if let Some(sample_rate) = data.sample_rate {
            sample_rate
        } else {
            return Err(Error::MissingSampleRate(id));
        };
        Ok(Self {
            bandwidth: data.bandwidth,
            uid: TrackUid::new(presentation_id, Self::TRACK_TYPE, switching_set_id, id),
            segments: data.segments,
            active_segment_id: data.active_segment_id,
            active_sequence_number: data.active_sequence_number,
            average_bandwidth: data.average_bandwidth,
            channels: data.channels,
            codecs,
            continuation_pattern: ContinuationPattern::new(base_url.clone(), continuation_pattern)?,
            frame_rate: data.frame_rate.unwrap_or_default(),
            label: data.label,
            initialization_pattern: InitializationPattern::new(base_url, initialization_pattern)?,
            media_time_offset: data.media_time_offset.unwrap_or_default(),
            sample_rate,
            segment_duration: data.segment_duration,
            transmission: data.transmission,
        })
    }
}
