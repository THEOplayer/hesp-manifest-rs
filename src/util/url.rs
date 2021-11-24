use url::Url;

use crate::Result;

pub trait RelativeUrl {
    fn resolve(&self, url: &Url) -> Result<Url>;
    fn make_relative(&mut self, url: &Url);
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

    fn make_relative(&mut self, url: &Url) {
        if let Some(relative_url) = self {
            relative_url.make_relative(url);
        }
    }
}

impl RelativeUrl for String {
    fn resolve(&self, url: &Url) -> Result<Url> {
        Ok(url.join(self)?)
    }

    fn make_relative(&mut self, url: &Url) {
        if let Ok(absolute) = Url::parse(self) {
            if let Some(relative) = url.make_relative(&absolute) {
                *self = relative;
            }
        }
    }
}
