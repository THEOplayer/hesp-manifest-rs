use serde::{Deserialize, Serialize};

use crate::*;

pub trait Track: Entity<Id = str> {
    fn active_segment(&self) -> Option<&Segment>;
    fn segment_duration(&self) -> Option<ScaledValue>;
    fn segments(&self) -> &[Segment];
    fn base_url(&self) -> &Option<RelativeBaseUrl>;
    fn base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl>;
    fn continuation_pattern(&self) -> &ContinuationPattern;
    fn set_continuation_pattern(&mut self, pattern: ContinuationPattern);
    fn average_bandwidth(&self) -> Option<f64>;
    fn segment(&self, segment_id: SegmentId) -> Option<&Segment> {
        self.segments()
            .iter()
            .find(|segment| segment.id() == segment_id)
    }
    fn duration_for_segment(&self, segment_id: SegmentId) -> Option<ScaledValue> {
        self.segment_duration().or_else(|| {
            self.segment(segment_id)
                .map(|segment| segment.duration().unwrap())
        })
    }
}

pub trait MediaTrack: Track {
    const MEDIA_TYPE: MediaType;
    fn bandwidth(&self) -> f64;
    fn initialization_pattern(&self) -> &InitializationPattern;
    fn set_initialization_pattern(&mut self, pattern: InitializationPattern);
    fn active_sequence_number(&self) -> Option<u64>;
    fn transmission(&self) -> &TrackTransmission;
    fn validate_active(&self) -> Result<()> {
        if self.active_sequence_number().is_none() {
            Err(Error::MissingActiveSequenceNumber(self.id().to_owned()))
        } else {
            Ok(())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct TransferObjectIdentifierLimits {
    pub start: u32,
    pub end: u32,
}

pub(crate) fn validate_segments(
    id: &str,
    duration: Option<ScaledValue>,
    segments: &[Segment],
) -> Result<()> {
    if duration.is_some() || segments.iter().all(|segment| segment.has_time_bounds()) {
        Ok(())
    } else {
        Err(Error::MissingSegmentDuration(id.to_owned()))
    }
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
            return Err($error($id));
        };
    };
}
