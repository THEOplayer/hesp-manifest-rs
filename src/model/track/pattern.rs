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

    pub fn into_full_pattern(self) -> String {
        if self.pattern.starts_with('/') || self.pattern.contains("://") {
            self.pattern
        } else {
            let base = match self.address.base_url() {
                RelativeUrl::Absolute(url) => url.join(".").unwrap().to_string(),
                //TODO this needs TESTING!!
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
}
