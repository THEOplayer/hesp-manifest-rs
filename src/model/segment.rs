use itertools::Itertools;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;

use crate::*;
use std::ops::Deref;
use std::borrow::Borrow;

#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Segment {
    id: u64,
    time_bounds: Option<TimeBounds>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Segments(Vec<Segment>);

impl Segments {
    fn new(vec: Vec<Segment>) -> Result<Self> {
        let jump = vec.iter()
            .map(Segment::id)
            .tuple_windows()
            .find(|&(a, b)| a + 1 != b);
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
    fn deref(&self) -> &[Segment] { &self.0 }
}

impl Borrow<[Segment]> for Segments {
    fn borrow(&self) -> &[Segment] { &self.0 }
}

impl Default for Segments {
    fn default() -> Self { Self(Vec::new()) }
}

impl Segment {
    pub fn id(&self) -> u64 {
        self.id
    }
    pub fn duration(&self) -> Option<ScaledValue> {
        self.time_bounds?.duration()
    }
    pub fn has_time_bounds(&self) -> bool {
        self.time_bounds.is_some()
    }
}