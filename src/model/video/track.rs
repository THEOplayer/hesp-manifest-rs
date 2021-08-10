use serde::{self, Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;
use crate::model::track::validate_segments;

#[skip_serializing_none]
#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoTrack {
    id: String,
    bandwidth: Number,
    resolution: Resolution,
    segments: Segments,
    active_segment: Option<u64>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    codecs: String,
    continuation_pattern: ContinuationPattern,
    frame_rate: ScaledValue,
    label: Option<String>,
    initialization_pattern: InitializationPattern,
    media_time_offset: ScaledValue,
    segment_duration: Option<ScaledValue>,
    toi_limits: Option<TransferObjectIdentifierLimits>,
}

impl Entity for VideoTrack {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}

impl Track for VideoTrack {
    fn active_segment(&self) -> Option<u64> { self.active_segment }
    fn segment_duration(&self) -> Option<ScaledValue> { self.segment_duration }
    fn segments(&self) -> &[Segment] { &self.segments }
    fn base_url(&self) -> &Option<RelativeBaseUrl> { &self.base_url }
    fn continuation_pattern(&self) -> &ContinuationPattern { &self.continuation_pattern }
    fn average_bandwidth(&self) -> Option<f64> {
        self.average_bandwidth.as_ref().and_then(Number::as_f64)
    }
}

impl MediaTrack for VideoTrack {
    fn bandwidth(&self) -> f64 { self.bandwidth.as_f64().unwrap() }
    fn initialization_pattern(&self) -> &InitializationPattern { &self.initialization_pattern }
    fn active_sequence_number(&self) -> Option<u64> { self.active_sequence_number }
    fn toi_limits(&self) -> Option<&TransferObjectIdentifierLimits> { self.toi_limits.as_ref() }
}

impl VideoTrack {
    pub(super) fn new(
        def: VideoTrackDef,
        default_codecs: Option<&String>,
        default_continuation_pattern: Option<&ContinuationPattern>,
        default_frame_rate: Option<ScaledValue>,
        default_initialization_pattern: Option<&InitializationPattern>,
        default_media_time_offset: ScaledValue,
    ) -> Result<Self> {
        let VideoTrackDef {
            bandwidth,
            id,
            resolution,
            segments,
            active_segment,
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
            toi_limits
        } = def;
        validate_segments(&id, segment_duration, &segments)?;
        default!(id, codecs, default_codecs, Error::MissingCodecs);
        default!(id, continuation_pattern, default_continuation_pattern, Error::MissingContinuationPattern);
        default!(id, frame_rate, default_frame_rate, Error::MissingFrameRate);
        default!(id, initialization_pattern, default_initialization_pattern, Error::MissingInitializationPattern);
        Ok(VideoTrack {
            bandwidth,
            id,
            resolution,
            segments,
            active_segment,
            active_sequence_number,
            average_bandwidth,
            base_url,
            codecs,
            continuation_pattern,
            frame_rate,
            label,
            initialization_pattern,
            media_time_offset: media_time_offset.unwrap_or(default_media_time_offset),
            segment_duration,
            toi_limits,
        })
    }
}


#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct VideoTrackDef {
    bandwidth: Number,
    id: String,
    resolution: Resolution,
    segments: Segments,
    active_segment: Option<u64>,
    active_sequence_number: Option<u64>,
    average_bandwidth: Option<Number>,
    base_url: Option<RelativeBaseUrl>,
    codecs: Option<String>,
    continuation_pattern: Option<ContinuationPattern>,
    frame_rate: Option<ScaledValue>,
    label: Option<String>,
    initialization_pattern: Option<InitializationPattern>,
    media_time_offset: Option<ScaledValue>,
    segment_duration: Option<ScaledValue>,
    toi_limits: Option<TransferObjectIdentifierLimits>,
}

impl Entity for VideoTrackDef {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}
