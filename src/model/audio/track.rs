use serde::{self, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::track::validate_segments;
use crate::*;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    bandwidth: Number,
    id: String,
    segments: Segments,
    active_segment: Option<u64>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    channels: Option<u64>,
    codecs: String,
    continuation_pattern: ContinuationPattern,
    frame_rate: u64,
    label: Option<String>,
    initialization_pattern: InitializationPattern,
    media_time_offset: ScaledValue,
    sample_rate: u64,
    segment_duration: Option<ScaledValue>,
    pub(crate) transmission: TrackTransmission,
}

impl Entity for AudioTrack {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}

impl Track for AudioTrack {
    fn active_segment(&self) -> Option<u64> {
        self.active_segment
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
        self.continuation_pattern = pattern
    }
    fn average_bandwidth(&self) -> Option<f64> {
        self.average_bandwidth.as_ref().and_then(Number::as_f64)
    }
}

impl MediaTrack for AudioTrack {
    const MEDIA_TYPE: MediaType = MediaType::Audio;
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
    pub(super) fn new(
        def: AudioTrackDef,
        default_codecs: Option<&String>,
        default_continuation_pattern: Option<&ContinuationPattern>,
        default_frame_rate: u64,
        default_initialization_pattern: Option<&InitializationPattern>,
        default_media_time_offset: ScaledValue,
        default_sample_rate: Option<u64>,
    ) -> Result<Self> {
        let AudioTrackDef {
            bandwidth,
            id,
            segments,
            active_segment,
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
        } = def;
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
            id,
            segments,
            active_segment,
            active_sequence_number,
            average_bandwidth,
            base_url,
            channels,
            codecs,
            continuation_pattern,
            frame_rate: frame_rate.unwrap_or(default_frame_rate),
            label,
            initialization_pattern,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            sample_rate,
            segment_duration,
            transmission,
        })
    }
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct AudioTrackDef {
    bandwidth: Number,
    id: String,
    segments: Segments,
    active_segment: Option<u64>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    channels: Option<u64>,
    codecs: Option<String>,
    continuation_pattern: Option<ContinuationPattern>,
    frame_rate: Option<u64>,
    label: Option<String>,
    initialization_pattern: Option<InitializationPattern>,
    media_time_offset: Option<ScaledValue>,
    sample_rate: Option<u64>,
    segment_duration: Option<ScaledValue>,
    transmission: TrackTransmission,
}

impl Entity for AudioTrackDef {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}
