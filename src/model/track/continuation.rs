use url::Url;

use crate::util::Uri;
use crate::{Address, Result, SegmentId, UrlPattern};

#[derive(Debug, Clone)]
pub struct ContinuationPattern(UrlPattern);

impl ContinuationPattern {
    const SEGMENT_ID_PATTERN: &'static str = "{segmentId}";

    pub fn new(address: Address, pattern: String) -> Result<Self> {
        UrlPattern::new(address, pattern, Self::SEGMENT_ID_PATTERN).map(Self)
    }

    #[must_use]
    pub fn segment(&self, id: SegmentId) -> Url {
        self.0.resolve(&id.to_string()).unwrap()
    }

    #[must_use]
    pub const fn base_url(&self) -> Option<&Uri> {
        self.0.base_url()
    }

    pub fn set_base_url(&mut self, base_url: Option<Uri>) -> Result<()> {
        self.0.set_base_url(base_url)
    }

    pub fn make_base_url_absolute(&mut self) {
        self.0.make_base_url_absolute();
    }

    #[must_use]
    pub fn into_pattern(self) -> String {
        self.0.into_pattern()
    }

    #[must_use]
    pub fn into_full_pattern(self) -> String {
        self.0.into_pattern_including_base_url()
    }

    pub fn set_pattern(&mut self, pattern: String) -> Result<()> {
        self.0.set_pattern(pattern)
    }
}
