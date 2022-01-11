use std::borrow::Cow;
use std::fmt;

use url::Url;

use crate::util::RelativeUrl;
use crate::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Address {
    manifest_location: Url,
    base_url: RelativeUrl,
}

impl Address {
    pub fn new(manifest_location: Url, base_url: RelativeUrl) -> Result<Address> {
        if let RelativeUrl::Path(path) = &base_url {
            manifest_location.join(path)?;
        };
        Ok(Address {
            manifest_location,
            base_url,
        })
    }

    pub fn join(&self, url: RelativeUrl) -> Result<Address> {
        Ok(Address {
            manifest_location: self.manifest_location.clone(),
            base_url: match (&self.base_url, &url) {
                (_, RelativeUrl::Absolute(_)) => url,
                (_, RelativeUrl::None) => self.base_url.clone(),
                (RelativeUrl::Absolute(absolute), RelativeUrl::Path(path)) => {
                    RelativeUrl::Absolute(absolute.join(path)?)
                }
                (RelativeUrl::Path(first), RelativeUrl::Path(second)) => RelativeUrl::Path(
                    self.manifest_location
                        .make_relative(&self.manifest_location.join(first)?.join(second)?)
                        .unwrap(),
                ),
                (RelativeUrl::None, RelativeUrl::Path(path)) => {
                    self.manifest_location.join(path)?;
                    url
                }
            },
        })
    }

    pub fn url(&self) -> Cow<Url> {
        match &self.base_url {
            RelativeUrl::Absolute(url) => Cow::Borrowed(url),
            RelativeUrl::Path(path) => Cow::Owned(self.manifest_location.join(path).unwrap()),
            RelativeUrl::None => Cow::Borrowed(&self.manifest_location),
        }
    }

    pub fn manifest_location(&self) -> &Url {
        &self.manifest_location
    }

    pub fn base_url(&self) -> &RelativeUrl {
        &self.base_url
    }

    pub fn set_absolute_base_url(&mut self, url: Url) {
        self.base_url = RelativeUrl::Absolute(url);
    }

    pub fn set_relative_base_url(&mut self, path: Option<String>) -> Result<()> {
        self.base_url = path.try_into()?;
        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.base_url {
            RelativeUrl::Absolute(url) => fmt::Display::fmt(url, f),
            RelativeUrl::Path(path) => {
                fmt::Display::fmt(&self.manifest_location.join(path).unwrap(), f)
            }
            RelativeUrl::None => fmt::Display::fmt(&self.manifest_location, f),
        }
    }
}

impl From<Address> for RelativeUrl {
    fn from(address: Address) -> Self {
        address.base_url
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn join_relative_urls() -> anyhow::Result<()> {
        let manifest_location = Url::parse("http://localhost/")?;
        let manifest_address = Address::new(manifest_location, RelativeUrl::None)?;
        let address = manifest_address
            .join(RelativeUrl::Path("audio/".to_owned()))?
            .join(RelativeUrl::Path("96k/".to_owned()))?;

        assert_eq!(address.to_string(), "http://localhost/audio/96k");
        Ok(())
    }
}
