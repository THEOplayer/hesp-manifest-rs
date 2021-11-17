use std::borrow::Borrow;
use std::ops::{Add, AddAssign, Deref, Sub, SubAssign};

use derive_more::{Display, From, Into};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[derive(
    Serialize,
    Deserialize,
    From,
    Into,
    Display,
    Debug,
    Clone,
    Copy,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
)]
pub struct SegmentId(u32);

#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    id: SegmentId,
    time_bounds: Option<TimeBounds>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Vec<Segment>")]
pub struct Segments(Vec<Segment>);

impl SegmentId {
    pub fn next(self) -> Self {
        self + 1
    }
}

impl TryFrom<Vec<Segment>> for Segments {
    type Error = Error;

    fn try_from(vec: Vec<Segment>) ->Result<Self> {
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

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn deserialize_checks_sequential_ids() -> Result<()> {
        let data = r#"
        [
           {"id": 10},
           {"id": 11},
           {"id": 13}
        ]"#;
        let result = serde_json::from_str::<Segments>(data);

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("13 must not follow 11"));
        Ok(())
    }

}