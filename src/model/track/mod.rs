pub use continuation::ContinuationPattern;
pub use initialization::*;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{MediaType, Result, ScaledValue, Segment, SegmentId, TrackTransmission};

mod continuation;
mod initialization;
mod uid;

pub trait Track: Entity {
    const TRACK_TYPE: MediaType;
    fn uid(&self) -> &TrackUid;
    fn bandwidth(&self) -> Option<f64>;
    fn active_segment(&self) -> Option<&Segment>;
    fn segment_duration(&self) -> Option<ScaledValue>;
    fn segments(&self) -> &[Segment];
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
    fn transmission(&self) -> &TrackTransmission;
    fn validate_active(&self) -> Result<()>;
}
