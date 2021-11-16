use url::Url;

use crate::util::{Entity, RelativeUrl};
use crate::*;
use super::data::AudioTrackData;

#[derive(Debug, Clone)]
pub struct AudioTrack {
    bandwidth: Number,
    uid: TrackUid,
    segments: Segments,
    active_segment_id: Option<SegmentId>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    channels: Option<u64>,
    codecs: String,
    continuation_pattern: ContinuationPattern,
    frame_rate: FrameRate,
    label: Option<String>,
    initialization_pattern: InitializationPattern,
    media_time_offset: ScaledValue,
    sample_rate: u64,
    segment_duration: Option<ScaledValue>,
    pub(crate) transmission: TrackTransmission,
}

impl Entity for AudioTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for AudioTrack {
    const TRACK_TYPE: TrackType = TrackType::Audio;

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
        self.continuation_pattern = pattern
    }
    fn average_bandwidth(&self) -> Option<f64> {
        self.average_bandwidth.as_ref().and_then(Number::as_f64)
    }
}

impl MediaTrack for AudioTrack {
    const MEDIA_TYPE: MediaType = MediaType::Audio;
    fn uid(&self) -> &TrackUid {
        &self.uid
    }
    fn bandwidth(&self) -> f64 {
        self.bandwidth.as_f64().unwrap()
    }
    fn initialization_pattern(&self) -> &InitializationPattern {
        &self.initialization_pattern
    }
    fn set_initialization_pattern(&mut self, pattern: InitializationPattern) {
        self.initialization_pattern = pattern;
    }
    fn active_sequence_number(&self) -> Option<u64> {
        self.active_sequence_number
    }
    fn transmission(&self) -> &TrackTransmission {
        &self.transmission
    }
}

impl AudioTrack {
    pub fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_url: &Url,
        data: AudioTrackData,
        default_codecs: Option<&str>,
        default_continuation_pattern: Option<&str>,
        default_frame_rate: FrameRate,
        default_initialization_pattern: Option<&str>,
        default_media_time_offset: ScaledValue,
        default_sample_rate: Option<u64>,
    ) -> Result<Self> {
        let AudioTrackData {
            bandwidth,
            id,
            segments,
            active_segment_id,
            active_sequence_number,
            average_bandwidth,
            base_url,
            channels,
            codecs,
            continuation_pattern,
            frame_rate,
            label,
            initialization_pattern,
            media_time_offset,
            sample_rate,
            segment_duration,
            transmission,
        } = data;
        let base_url = base_url.resolve(switching_set_url)?;
        default!(id, codecs, default_codecs, Error::MissingCodecs);
        default!(
            id,
            continuation_pattern,
            default_continuation_pattern,
            Error::MissingContinuationPattern
        );
        default!(
            id,
            initialization_pattern,
            default_initialization_pattern,
            Error::MissingInitializationPattern
        );
        default!(
            id,
            sample_rate,
            default_sample_rate,
            Error::MissingSampleRate
        );
        validate_segments(&id, segment_duration, &segments)?;
        Ok(AudioTrack {
            bandwidth,
            uid: TrackUid::new(presentation_id, Self::TRACK_TYPE, switching_set_id, id),
            segments,
            active_segment_id,
            active_sequence_number,
            average_bandwidth,
            channels,
            codecs,
            continuation_pattern: ContinuationPattern::new(base_url.clone(), continuation_pattern)?,
            frame_rate: frame_rate.unwrap_or(default_frame_rate),
            label,
            initialization_pattern: InitializationPattern::new(base_url, initialization_pattern)?,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            sample_rate,
            segment_duration,
            transmission,
        })
    }
}