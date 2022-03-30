use std::borrow::Cow;
use std::fmt;

use url::Url;

use crate::util::Uri;
use crate::Result;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Address {
    manifest_location: Url,
    uri: Option<Uri>,
}

impl Address {
    pub fn new(manifest_location: Url, base_url: Option<Uri>) -> Result<Address> {
        if manifest_location.cannot_be_a_base() {
            return Err(url::ParseError::RelativeUrlWithCannotBeABaseBase.into());
        }
        if let Some(Uri::Relative(path)) = &base_url {
            manifest_location.join(path)?;
        };
        Ok(Address {
            manifest_location,
            uri: base_url,
        })
    }

    pub fn join(&self, uri: Option<Uri>) -> Result<Address> {
        Ok(Address {
            manifest_location: self.manifest_location.clone(),
            uri: match (&self.uri, &uri) {
                (_, Some(Uri::Absolute(_))) => uri,
                (_, None) => self.uri.clone(),
                (Some(Uri::Absolute(absolute_url)), Some(Uri::Relative(path))) => {
                    Some(Uri::Absolute(absolute_url.join(path)?))
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

    #[must_use]
    pub fn url(&self) -> Cow<Url> {
        match &self.uri {
            Some(Uri::Absolute(url)) => Cow::Borrowed(url),
            Some(Uri::Relative(path)) => Cow::Owned(self.manifest_location.join(path).unwrap()),
            None => Cow::Borrowed(&self.manifest_location),
        }
    }

    #[must_use]
    pub fn manifest_location(&self) -> &Url {
        &self.manifest_location
    }

    #[must_use]
    pub fn uri(&self) -> Option<&Uri> {
        self.uri.as_ref()
    }

    pub fn set_uri(&mut self, uri: Option<Uri>) -> Result<()> {
        if let Some(Uri::Relative(path)) = &uri {
            self.manifest_location.join(path)?;
        };
        self.uri = uri;
        Ok(())
    }

    pub fn make_absolute(&mut self) {
        self.uri = Some(Uri::Absolute(self.url().into_owned()));
    }
}

impl fmt::Display for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.uri {
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
        address.uri
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

    #[test]
    fn make_relative_url_absolute() -> Result<()> {
        let base_url = Some(Uri::Relative("bar/abc".to_string()));
        let mut address = Address::new("http://localhost/foo/".try_into()?, base_url)?;

        address.make_absolute();

        assert_eq!(
            address.uri(),
            Some(&Uri::Absolute("http://localhost/foo/bar/abc".try_into()?))
        );

        Ok(())
    }
}
