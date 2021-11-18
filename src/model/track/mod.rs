use serde::{Deserialize, Serialize};

pub use track_type::TrackType;
pub use uid::TrackUid;

use crate::util::Entity;
use crate::{
    ContinuationPattern, InitializationPattern, MediaType, ScaledValue, Segment, SegmentId,
    TrackTransmission,
};

mod track_type;
mod uid;

pub trait Track: Entity {
    const TRACK_TYPE: TrackType;

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
}

pub trait MediaTrack: Track {
    //TODO check if this is still needed now we have TrackType
    const MEDIA_TYPE: MediaType;
    fn uid(&self) -> &TrackUid;
    fn bandwidth(&self) -> f64;
    fn initialization_pattern(&self) -> &InitializationPattern;
    fn set_initialization_pattern(&mut self, pattern: InitializationPattern);
    fn active_sequence_number(&self) -> Option<u64>;
    fn transmission(&self) -> &TrackTransmission;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Copy, Serialize, Deserialize)]
pub struct TransferObjectIdentifierLimits {
    pub start: u32,
    pub end: u32,
}
