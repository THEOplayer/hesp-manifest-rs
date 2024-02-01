pub use continuation::ContinuationPattern;
pub use initialization::*;
pub use pattern::UrlPattern;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{MediaType, ScaledDuration, Segment, SegmentId, UnsignedScaledValue};

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

    #[allow(clippy::option_if_let_else)]
    #[allow(clippy::cast_precision_loss)]
    fn active_segment(&self, time: f64) -> Option<&Segment> {
        let segments = self.segments();
        if segments.is_empty() {
            return None;
        }
        let first_segment = segments.first().unwrap();
        let id_diff = u64::from(first_segment.id()) - u64::from(self.start_segment_id());
        let default_duration = self.segment_duration().map(ScaledDuration::to_secs);
        let mut current_end = first_segment
            .time_bounds()
            .and_then(|x| x.start_time())
            .map_or(
                id_diff as f64 * default_duration.unwrap(),
                UnsignedScaledValue::to_secs,
            );
        for segment in segments {
            let (start, end) = if let Some(time_bounds) = segment.time_bounds() {
                let start = time_bounds
                    .start_time()
                    .map_or(current_end, UnsignedScaledValue::to_secs);
                let end = time_bounds.end_time().map_or(
                    current_end + default_duration.unwrap(),
                    UnsignedScaledValue::to_secs,
                );
                (start, end)
            } else {
                let start = current_end;
                let end = current_end + default_duration.unwrap();
                (start, end)
            };
            if start <= time && time <= end {
                return Some(segment);
            }
            current_end = end;
        }
        None
    }
    fn start_segment_id(&self) -> SegmentId;

    fn start_segment(&self) -> Option<&Segment> {
        self.segment(self.start_segment_id())
    }

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
