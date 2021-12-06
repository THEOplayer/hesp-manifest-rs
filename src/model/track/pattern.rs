use url::Url;

use crate::{Error, Result};

#[derive(Debug, Clone)]
pub struct UrlPattern {
    base: Url,
    pattern: String,
    placeholder: &'static str,
}

impl UrlPattern {
    pub fn new(base: &Url, pattern: String, placeholder: &'static str) -> Result<Self> {
        base.join(&pattern)?;
        if pattern.contains(placeholder) {
            Ok(Self {
                base: base.join(".").unwrap(),
                pattern,
                placeholder,
            })
        } else {
            Err(Error::InvalidPattern(pattern, placeholder))
        }
    }

    pub fn resolve(&self, input: &str) -> Result<Url> {
        let rel = self.pattern.replace(self.placeholder, input);
        Ok(self.base.join(&rel)?)
    }

    pub fn make_relative(&self, url: &Url) -> String {
        if self.pattern.starts_with('/') || self.pattern.contains("://") {
            self.pattern.clone()
        } else {
            let base = url
                .make_relative(&self.base)
                .unwrap_or_else(|| self.base.to_string());
            format!("{}{}", base, self.pattern)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn relative_pattern() {
        let base = &Url::parse("https://example.com/s2/audio/").unwrap();
        let relative = UrlPattern::new(base, "128k-init-{initId}.mp4".to_owned(), "{initId}")
            .unwrap()
            .make_relative(&Url::parse("https://example.com/manifest.json").unwrap());

        assert_eq!(relative, "s2/audio/128k-init-{initId}.mp4");
    }

    #[test]
    fn pattern_relative_to_foreign_url() {
        let base = &Url::parse("https://example.com/s2/audio/").unwrap();
        let relative = UrlPattern::new(base, "128k-init-{initId}.mp4".to_owned(), "{initId}")
            .unwrap()
            .make_relative(&Url::parse("https://foreign.com/manifest.json").unwrap());

        assert_eq!(
            relative,
            "https://example.com/s2/audio/128k-init-{initId}.mp4"
        );
    }
}
