pub use continuation::ContinuationPattern;
pub use initialization::*;
pub use pattern::UrlPattern;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{MediaType, Result, ScaledDuration, Segment, SegmentId, TrackTransmission};

mod continuation;
mod initialization;
mod pattern;
mod uid;

pub trait Track: Entity {
    const TRACK_TYPE: MediaType;
    fn uid(&self) -> &TrackUid;
    fn bandwidth(&self) -> Option<u64>;
    fn active_segment(&self) -> Option<&Segment>;
    fn segment_duration(&self) -> Option<ScaledDuration>;
    fn segments(&self) -> &[Segment];
    fn continuation_pattern(&self) -> &ContinuationPattern;
    fn set_continuation_pattern(&mut self, pattern: ContinuationPattern);
    fn average_bandwidth(&self) -> Option<u64>;
    fn segment(&self, segment_id: SegmentId) -> Option<&Segment> {
        self.segments()
            .iter()
            .find(|segment| segment.id() == segment_id)
    }
    fn duration_for_segment(&self, segment_id: SegmentId) -> Option<ScaledDuration> {
        self.segment_duration().or_else(|| {
            self.segment(segment_id)
                .map(|segment| segment.duration().unwrap())
        })
    }
    fn transmission(&self) -> &TrackTransmission;
    fn validate_active(&self) -> Result<()>;
}
