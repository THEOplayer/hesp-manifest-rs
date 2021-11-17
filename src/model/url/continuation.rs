use std::fmt;
use url::Url;

use crate::*;

#[derive(Debug, Clone)]
pub struct ContinuationPattern {
    base: Url,
    pattern: String,
}

impl ContinuationPattern {
    const SEGMENT_ID_PATTERN: &'static str = "{segmentId}";

    pub fn new(base: Url, pattern: String) -> Result<Self> {
        base.join(&pattern)?;
        if pattern.contains(Self::SEGMENT_ID_PATTERN) {
            Ok(Self { base, pattern })
        } else {
            Err(Error::InvalidContinuationPattern(pattern))
        }
    }

    pub fn segment(&self, id: SegmentId) -> Url {
        let rel = self
            .pattern
            .replace(Self::SEGMENT_ID_PATTERN, &id.to_string());
        self.base.join(&rel).unwrap()
    }
}

impl fmt::Display for ContinuationPattern {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.base
            .join(&self.pattern)
            .unwrap()
            .to_string()
            .replace("%7BsegmentId%7D", Self::SEGMENT_ID_PATTERN)
            .fmt(f)
    }
}
