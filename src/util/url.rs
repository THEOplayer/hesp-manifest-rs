use url::Url;
use crate::*;

pub trait RelativeUrl {
    fn resolve(&self, url: &Url) -> Result<Url>;
}

impl RelativeUrl for Option<String> {
    fn resolve(&self, url: &Url) -> Result<Url> {
        if let Some(relative_url) = self {
            relative_url.resolve(url)
        } else {
            Ok(url.clone())
        }
    }
}

impl RelativeUrl for String {
    fn resolve(&self, url: &Url) -> Result<Url> {
        Ok(url.join(self)?)
    }
}