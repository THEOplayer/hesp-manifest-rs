use url::Url;

use crate::model::track::validate_segments;
use crate::*;
use crate::util::{Entity, RelativeUrl};

#[derive(Debug, Clone)]
pub struct VideoTrack {
    uid: TrackUid,
    bandwidth: Number,
    resolution: Resolution,
    segments: Segments,
    active_segment_id: Option<SegmentId>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    codecs: String,
    continuation_pattern: ContinuationPattern,
    frame_rate: ScaledValue,
    label: Option<String>,
    initialization_pattern: InitializationPattern,
    media_time_offset: ScaledValue,
    segment_duration: Option<ScaledValue>,
    pub(crate) transmission: TrackTransmission,
}

impl Entity for VideoTrack {
    fn id(&self) -> &str {
        self.uid.track_id()
    }
}

impl Track for VideoTrack {
    const TRACK_TYPE: TrackType = TrackType::Video;

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

impl MediaTrack for VideoTrack {
    const MEDIA_TYPE: MediaType = MediaType::Video;

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

impl VideoTrack {
    pub(super) fn new(
        presentation_id: String,
        switching_set_id: String,
        switching_set_url: &Url,
        data: VideoTrackData,
        default_codecs: Option<&str>,
        default_continuation_pattern: Option<&str>,
        default_frame_rate: Option<ScaledValue>,
        default_initialization_pattern: Option<&str>,
        default_media_time_offset: ScaledValue,
    ) -> Result<Self> {
        let VideoTrackData {
            bandwidth,
            id,
            resolution,
            segments,
            active_segment_id,
            active_sequence_number,
            average_bandwidth,
            base_url,
            codecs,
            continuation_pattern,
            frame_rate,
            label,
            initialization_pattern,
            media_time_offset,
            segment_duration,
            transmission,
        } = data;
        let base_url = base_url.resolve(switching_set_url)?;
        validate_segments(&id, segment_duration, &segments)?;
        default!(id, codecs, default_codecs, Error::MissingCodecs);
        default!(
            id,
            continuation_pattern,
            default_continuation_pattern,
            Error::MissingContinuationPattern
        );
        default!(id, frame_rate, default_frame_rate, Error::MissingFrameRate);
        default!(
            id,
            initialization_pattern,
            default_initialization_pattern,
            Error::MissingInitializationPattern
        );
        Ok(VideoTrack {
            bandwidth,
            uid: TrackUid::new(presentation_id, Self::TRACK_TYPE, switching_set_id, id),
            resolution,
            segments,
            active_segment_id,
            active_sequence_number,
            average_bandwidth,
            codecs,
            continuation_pattern: ContinuationPattern::new(base_url.clone(), continuation_pattern)?,
            frame_rate,
            label,
            initialization_pattern: InitializationPattern::new(base_url, initialization_pattern)?,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            segment_duration,
            transmission,
        })
    }
}
