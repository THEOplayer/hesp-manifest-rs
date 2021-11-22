use url::Url;

use crate::Result;

pub trait RelativeUrl {
    fn resolve(&self, url: &Url) -> Result<Url>;
}

impl RelativeUrl for Option<String> {
    #[allow(clippy::option_if_let_else)]
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
