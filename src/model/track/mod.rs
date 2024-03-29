pub use continuation::ContinuationPattern;
pub use initialization::*;
pub use pattern::UrlPattern;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{MediaType, ScaledDuration, Segment, SegmentId};

mod continuation;
mod initialization;
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

    fn start_segment_id(&self) -> SegmentId;
    #[deprecated(note = "please use `start_segment_id` instead")]
    fn active_segment_id(&self) -> Option<SegmentId>;
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
}

pub trait InitializableTrack: Track + Initialization + Send {}

impl<T: Track + Initialization + Send> InitializableTrack for T {}
