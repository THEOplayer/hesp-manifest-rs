use std::borrow::Borrow;
use std::fmt;
use std::fmt::Debug;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::UInt;
use crate::{Error, Result, ScaledDuration, TimeBounds};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct SegmentId(#[serde(deserialize_with = "UInt::deserialize_u64")] u64);

impl SegmentId {
    pub fn next(self) -> Self {
        self + 1
    }
}

impl From<u64> for SegmentId {
    fn from(segment_id: u64) -> Self {
        SegmentId(segment_id)
    }
}

impl From<SegmentId> for u64 {
    fn from(segment_id: SegmentId) -> Self {
        segment_id.0
    }
}

impl Add<u64> for SegmentId {
    type Output = Self;
    fn add(self, rhs: u64) -> Self {
        Self(self.0 + rhs)
    }
}

impl Sub<u64> for SegmentId {
    type Output = Self;
    fn sub(self, rhs: u64) -> Self {
        Self(self.0 - rhs)
    }
}

impl AddAssign<u64> for SegmentId {
    fn add_assign(&mut self, rhs: u64) {
        self.0 += rhs;
    }
}

impl SubAssign<u64> for SegmentId {
    fn sub_assign(&mut self, rhs: u64) {
        self.0 -= rhs;
    }
}

impl fmt::Display for SegmentId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    id: SegmentId,
    time_bounds: Option<TimeBounds>,
}

impl Segment {
    pub const fn id(&self) -> SegmentId {
        self.id
    }
    pub fn duration(&self) -> Option<ScaledDuration> {
        self.time_bounds?.duration()
    }
    pub const fn has_time_bounds(&self) -> bool {
        self.time_bounds.is_some()
    }
}

impl TryFrom<Vec<Segment>> for Segments {
    type Error = Error;

    fn try_from(vec: Vec<Segment>) -> Result<Self> {
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(try_from = "Vec<Segment>")]
pub struct Segments(Vec<Segment>);

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_checks_sequential_ids() {
        let data = r#"
        [
           {"id": 10},
           {"id": 11},
           {"id": 13}
        ]"#;
        let result = serde_json::from_str::<Segments>(data);

        assert!(result.is_err());
        assert!(result
            .unwrap_err()
            .to_string()
            .contains("13 must not follow 11"));
    }
}
