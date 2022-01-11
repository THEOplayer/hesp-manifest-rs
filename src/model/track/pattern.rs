use url::Url;

use crate::util::RelativeUrl;
use crate::{Address, Error, Result};

#[derive(Debug, Clone)]
pub struct UrlPattern {
    address: Address,
    pattern: String,
    placeholder: &'static str,
}

impl UrlPattern {
    pub fn new(address: Address, pattern: String, placeholder: &'static str) -> Result<Self> {
        if pattern.contains(placeholder) {
            Ok(Self {
                address,
                pattern,
                placeholder,
            })
        } else {
            Err(Error::InvalidPattern(pattern, placeholder))
        }
    }

    pub fn resolve(&self, input: &str) -> Result<Url> {
        let path = &self.pattern.replace(self.placeholder, input);
        Ok(self.address.url().join(path)?)
    }

    pub fn base_url(&self) -> &RelativeUrl {
        self.address.base_url()
    }

    pub fn into_pattern(self) -> String {
        self.pattern
    }

    pub fn into_pattern_including_address(self) -> String {
        if self.pattern.starts_with('/') || self.pattern.contains("://") {
            self.pattern
        } else {
            let base = match self.address.base_url() {
                RelativeUrl::Absolute(url) => url.join(".").unwrap().to_string(),
                RelativeUrl::Path(path) => {
                    let absolute = self
                        .address
                        .manifest_location()
                        .join(path)
                        .unwrap()
                        .join(".")
                        .unwrap();
                    self.address
                        .manifest_location()
                        .make_relative(&absolute)
                        .unwrap()
                }
                RelativeUrl::None => return self.pattern,
            };
            format!("{}{}", base, self.pattern)
        }
    }

    pub fn set_pattern(&mut self, pattern: String) -> Result<()> {
        if pattern.contains(self.placeholder) {
            self.pattern = pattern;
            Ok(())
        } else {
            Err(Error::InvalidPattern(pattern, self.placeholder))
        }
    }

    pub fn set_absolute_base_url(&mut self, url: Url) {
        self.address.set_absolute_base_url(url);
    }

    pub fn set_relative_base_url(&mut self, path: Option<String>) -> Result<()> {
        self.address.set_relative_base_url(path)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn include_relative_url_into_pattern() -> Result<()> {
        let manifest_location = Url::parse("http://localhost/whatever")?;
        let base_url = RelativeUrl::Path(String::from("bar/will-be-deleted"));
        let url_pattern = UrlPattern::new(
            Address::new(manifest_location, base_url)?,
            String::from("some-{xxx}.xxx"),
            "{xxx}",
        )?;

        assert_eq!(
            url_pattern.into_pattern_including_address(),
            "bar/some-{xxx}.xxx"
        );
        Ok(())
    }
}
