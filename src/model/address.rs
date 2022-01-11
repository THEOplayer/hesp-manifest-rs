use std::borrow::Cow;
use std::fmt;

use url::Url;

use crate::util::Uri;
use crate::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Address {
    manifest_location: Url,
    base_url: Option<Uri>,
}

impl Address {
    pub fn new(manifest_location: Url, base_url: Option<Uri>) -> Result<Address> {
        if let Some(Uri::Relative(path)) = &base_url {
            manifest_location.join(path)?;
        };
        Ok(Address {
            manifest_location,
            base_url,
        })
    }

    pub fn join(&self, uri: Option<Uri>) -> Result<Address> {
        Ok(Address {
            manifest_location: self.manifest_location.clone(),
            base_url: match (&self.base_url, &uri) {
                (_, Some(Uri::Absolute(_))) => uri,
                (_, None) => self.base_url.clone(),
                (Some(Uri::Absolute(absolute)), Some(Uri::Relative(path))) => {
                    Some(Uri::Absolute(absolute.join(path)?))
                }
                (Some(Uri::Relative(first)), Some(Uri::Relative(second))) => Some(Uri::Relative(
                    self.manifest_location
                        .make_relative(&self.manifest_location.join(first)?.join(second)?)
                        .unwrap(),
                )),
                (None, Some(Uri::Relative(path))) => {
                    self.manifest_location.join(path)?;
                    uri
                }
            },
        })
    }

    pub fn url(&self) -> Cow<Url> {
        match &self.base_url {
            Some(Uri::Absolute(url)) => Cow::Borrowed(url),
            Some(Uri::Relative(path)) => Cow::Owned(self.manifest_location.join(path).unwrap()),
            None => Cow::Borrowed(&self.manifest_location),
        }
    }

    pub fn manifest_location(&self) -> &Url {
        &self.manifest_location
    }

    pub fn base_url(&self) -> Option<&Uri> {
        self.base_url.as_ref()
    }

    pub fn set_base_url(&mut self, base_url: Option<Uri>) -> Result<()> {
        if let Some(Uri::Relative(path)) = &base_url {
            self.manifest_location.join(path)?;
        };
        self.base_url = base_url;
        Ok(())
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.base_url {
            Some(Uri::Absolute(url)) => fmt::Display::fmt(url, f),
            Some(Uri::Relative(path)) => {
                fmt::Display::fmt(&self.manifest_location.join(path).unwrap(), f)
            }
            None => fmt::Display::fmt(&self.manifest_location, f),
        }
    }
}

impl From<Address> for Option<Uri> {
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
        let manifest_address = Address::new(manifest_location, None)?;
        let address = manifest_address
            .join(Some(Uri::Relative("audio/".to_owned())))?
            .join(None)?
            .join(Some(Uri::Relative("96k/".to_owned())))?;

        assert_eq!(address.to_string(), "http://localhost/audio/96k");
        Ok(())
    }
}
