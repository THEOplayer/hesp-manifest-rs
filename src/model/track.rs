use std::ops::Range;

use crate::*;

pub trait Track: Entity<Id=str> {
    fn active_segment(&self) -> Option<u64>;
    fn segment_duration(&self) -> Option<ScaledValue>;
    fn segments(&self) -> &[Segment];
    fn base_url(&self) -> &Option<RelativeBaseUrl>;
    fn continuation_pattern(&self) -> &ContinuationPattern;
    fn average_bandwidth(&self) -> Option<f64>;
    fn get_segment(&self, segment_id: u64) -> Option<&Segment> {
        self.segments().iter().find(|segment| segment.id() == segment_id)
    }
    fn get_segment_duration(&self, segment_id: u64) -> ScaledValue {
        self.segment_duration().or_else(|| {
            self.get_segment(segment_id).and_then(|segment| segment.duration())
        }).unwrap_or_else(|| ScaledValue::new(20))
        //TODO should panic instead of returning 20 secs
    }
}

pub trait MediaTrack: Track {
    const MEDIA_TYPE: MediaType;
    fn bandwidth(&self) -> f64;
    fn initialization_pattern(&self) -> &InitializationPattern;
    fn active_sequence_number(&self) -> Option<u64>;
    fn transmission(&self) -> &TrackTransmission;
}

pub type TransferObjectIdentifierLimits = Range<u32>;

pub(crate) fn validate_segments(_id: &str, _duration: Option<ScaledValue>, _segments: &[Segment]) -> Result<()> {
    // TODO uncomment
    // if  duration.is_some() || segments.iter().all(|segment| segment.has_time_bounds()) {
    //     Err(Error::MissingSegmentDuration(id.to_owned()))
    // } else {
    Ok(())
    // }
}

#[macro_export]
/// defaults 2 optionals or return an error when not possible
macro_rules! default {
    ($id:expr, $var: ident, $default:expr, $error: expr) => {
        let $var = if let Some(value) = $var {
            value
        } else if let Some(value) = $default {
            value.clone()
        } else {
            return Err($error($id))
        };
    };
}