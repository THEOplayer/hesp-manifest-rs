use url::Url;

use crate::{Result, SegmentId, UrlPattern};

#[derive(Debug, Clone)]
pub struct ContinuationPattern(UrlPattern);

impl ContinuationPattern {
    const SEGMENT_ID_PATTERN: &'static str = "{segmentId}";

    pub fn new(base: &Url, pattern: String) -> Result<Self> {
        UrlPattern::new(base, pattern, Self::SEGMENT_ID_PATTERN).map(Self)
    }

    pub fn segment(&self, id: SegmentId) -> Url {
        self.0.resolve(&id.to_string()).unwrap()
    }

    pub fn make_relative(&self, url: &Url) -> String {
        self.0.make_relative(url)
    }
}
