use url::Url;

use crate::util::Uri;
use crate::{Address, Error, Result};

#[derive(Debug, Clone)]
pub struct UrlPattern {
    base_address: Address,
    pattern: String,
    placeholder: &'static str,
}

impl UrlPattern {
    pub fn new(address: Address, pattern: String, placeholder: &'static str) -> Result<Self> {
        let result = Self {
            base_address: address,
            pattern,
            placeholder,
        };
        result.validate()?;
        Ok(result)
    }

    pub fn resolve(&self, input: &str) -> Result<Url> {
        let path = &self.pattern.replace(self.placeholder, input);
        Ok(self.base_address.url().join(path)?)
    }

    #[must_use]
    pub fn into_pattern(self) -> String {
        self.pattern
    }

    #[must_use]
    pub fn into_pattern_including_base_url(self) -> String {
        if self.pattern.starts_with('/') || self.pattern.contains("://") {
            self.pattern
        } else {
            let base = match self.base_address.uri() {
                Some(Uri::Absolute(url)) => url.join(".").unwrap().to_string(),
                Some(Uri::Relative(path)) => {
                    let absolute = self
                        .base_address
                        .manifest_location()
                        .join(path)
                        .unwrap()
                        .join(".")
                        .unwrap();
                    self.base_address
                        .manifest_location()
                        .make_relative(&absolute)
                        .unwrap()
                }
                None => return self.pattern,
            };
            format!("{}{}", base, self.pattern)
        }
    }

    pub fn set_pattern(&mut self, pattern: String) -> Result<()> {
        self.pattern = pattern;
        self.validate()
    }

    #[must_use]
    pub const fn base_url(&self) -> Option<&Uri> {
        self.base_address.uri()
    }

    pub fn set_base_url(&mut self, base_url: Option<Uri>) -> Result<()> {
        self.base_address.set_uri(base_url)
    }

    pub fn make_base_url_absolute(&mut self) {
        self.base_address.make_absolute();
    }

    fn validate(&self) -> Result<()> {
        if !self.pattern.contains(self.placeholder) {
            return Err(Error::InvalidPattern(
                self.pattern.clone(),
                self.placeholder,
            ));
        }
        self.resolve("")?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn include_relative_url_into_pattern() -> Result<()> {
        let manifest_location = Url::parse("http://localhost/whatever")?;
        let base_url = Some(Uri::Relative(String::from("bar/will-be-deleted")));
        let url_pattern = UrlPattern::new(
            Address::new(manifest_location, base_url)?,
            String::from("some-{xxx}.xxx"),
            "{xxx}",
        )?;

        assert_eq!(
            url_pattern.into_pattern_including_base_url(),
            "bar/some-{xxx}.xxx"
        );
        Ok(())
    }
}
