pub use continuation::ContinuationPattern;
pub use initialization::*;
pub use multicast::*;
pub use pattern::UrlPattern;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{MediaType, Result, ScaledDuration, Segment, SegmentId};

mod continuation;
mod initialization;
mod multicast;
mod pattern;
mod uid;

pub trait Track: Entity {
    fn uid(&self) -> &TrackUid;

    fn segment(&self, segment_id: SegmentId) -> Option<&Segment> {
        self.segments()
            .iter()
            .find(|segment| segment.id() == segment_id)
    }
    fn segments(&self) -> &[Segment];
    fn active_segment(&self) -> Option<&Segment>;
    fn segment_duration(&self) -> Option<ScaledDuration>;
    fn duration_for_segment(&self, segment_id: SegmentId) -> Option<ScaledDuration> {
        self.segment_duration().or_else(|| {
            self.segment(segment_id)
                .map(|segment| segment.duration().unwrap())
        })
    }

    fn average_bandwidth(&self) -> Option<u64>;
    fn bandwidth(&self) -> Option<u64>;

    fn continuation_pattern(&self) -> &ContinuationPattern;
    fn continuation_pattern_mut(&mut self) -> &mut ContinuationPattern;

    fn media_type(&self) -> MediaType;
    fn mime_type(&self) -> &str;
    fn transmission(&self) -> &TrackTransmission;
}

pub(crate) trait ValidateTrack: Track {
    fn validate_active(&self) -> Result<()>;
}

pub trait InitializableTrack: Track + Initialization {}
impl<T: Track + Initialization> InitializableTrack for T {}
