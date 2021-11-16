use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

use derive_more::{Display, From, Into};
use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[derive(
    Serialize,
    Deserialize,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    From,
    Into,
    Display,
)]
pub struct SegmentId(u32);

#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    id: SegmentId,
    time_bounds: Option<TimeBounds>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Segments(Vec<Segment>);

impl SegmentId {
    pub fn next(self) -> Self {
        self + 1
    }
}

impl Segments {
    //TODO use serde try_from
    fn new(vec: Vec<Segment>) -> Result<Self> {
        let jump = vec
            .iter()
            .map(Segment::id)
            .tuple_windows()
            .find(|&(a, b)| a.next() != b);
        if let Some((a, b)) = jump {
            Err(Error::InvalidSegmentIds(a, b))
        } else {
            Ok(Self(vec))
        }
    }
}

impl<'de> Deserialize<'de> for Segments {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let vec = Vec::deserialize(deserializer)?;
        Self::new(vec).map_err(serde::de::Error::custom)
    }
}

impl Deref for Segments {
    type Target = [Segment];
    fn deref(&self) -> &[Segment] {
        &self.0
    }
}

impl Borrow<[Segment]> for Segments {
    fn borrow(&self) -> &[Segment] {
        &self.0
    }
}

impl Default for Segments {
    fn default() -> Self {
        Self(Vec::new())
    }
}

impl Segment {
    pub fn id(&self) -> SegmentId {
        self.id
    }
    pub fn duration(&self) -> Option<ScaledValue> {
        self.time_bounds?.duration()
    }
    pub fn has_time_bounds(&self) -> bool {
        self.time_bounds.is_some()
    }
}

impl Add<u32> for SegmentId {
    type Output = SegmentId;
    fn add(self, rhs: u32) -> Self {
        SegmentId(self.0 + rhs)
    }
}

impl Sub<u32> for SegmentId {
    type Output = SegmentId;
    fn sub(self, rhs: u32) -> Self {
        SegmentId(self.0 - rhs)
    }
}

impl AddAssign<u32> for SegmentId {
    fn add_assign(&mut self, rhs: u32) {
        self.0 += rhs
    }
}

impl SubAssign<u32> for SegmentId {
    fn sub_assign(&mut self, rhs: u32) {
        self.0 -= rhs
    }
}
