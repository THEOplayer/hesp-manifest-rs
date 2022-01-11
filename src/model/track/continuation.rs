use url::Url;

use crate::util::RelativeUrl;
use crate::{Address, Result, SegmentId, UrlPattern};

#[derive(Debug, Clone)]
pub struct ContinuationPattern(UrlPattern);

impl ContinuationPattern {
    const SEGMENT_ID_PATTERN: &'static str = "{segmentId}";

    pub fn new(address: Address, pattern: String) -> Result<Self> {
        UrlPattern::new(address, pattern, Self::SEGMENT_ID_PATTERN).map(Self)
    }

    pub fn segment(&self, id: SegmentId) -> Url {
        self.0.resolve(&id.to_string()).unwrap()
    }

    pub fn base_url(&self) -> &RelativeUrl {
        self.0.base_url()
    }

    pub fn into_pattern(self) -> String {
        self.0.into_pattern()
    }

    pub fn into_full_pattern(self) -> String {
        self.0.into_pattern_including_address()
    }

    pub fn set_pattern(&mut self, pattern: String) -> Result<()> {
        self.0.set_pattern(pattern)
    }

    pub fn set_absolute_base_url(&mut self, url: Url) {
        self.0.set_absolute_base_url(url);
    }

    pub fn set_relative_base_url(&mut self, path: Option<String>) -> Result<()> {
        self.0.set_relative_base_url(path)
    }
}
