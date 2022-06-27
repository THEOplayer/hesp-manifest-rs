use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TimeSource {
    pub scheme: Url,
    pub url: Url,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn serialize_time_source() -> Result<()> {
        let source = TimeSource {
            scheme: Url::parse("urn:mpeg:dash:utc:ntp:2014")?,
            url: Url::parse("https://xxx")?,
        };
        let json = serde_json::to_string(&source)?;
        assert_eq!(
            json,
            r#"{"scheme":"urn:mpeg:dash:utc:ntp:2014","url":"https://xxx/"}"#
        );
        Ok(())
    }

    #[test]
    fn deserialize_time_source() -> Result<()> {
        let data = r#"
        {
            "scheme": "urn:mpeg:dash:utc:ntp:2014",
            "url": "https://xxx"
        }"#;
        let TimeSource { scheme, url } = serde_json::from_str(data)?;

        assert_eq!(scheme, Url::parse("urn:mpeg:dash:utc:ntp:2014")?);
        assert_eq!(url, Url::parse("https://xxx")?);
        Ok(())
    }

    #[test]
    fn invalid_scheme_in_time_source() {
        let data = r#"
        {
            "scheme": "John Doe",
            "url": "https://xxx"
        }"#;
        let result = serde_json::from_str::<TimeSource>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("expected relative URL"),
            "Error did not indicate scheme parsing failed `{}`",
            error
        );
    }

    #[test]
    fn invalid_url_in_time_source() {
        let data = r#"
        {
            "scheme": "urn:mpeg:dash:utc:ntp:2014",
            "url": 43
        }"#;
        let result = serde_json::from_str::<TimeSource>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("expected a string representing an URL"),
            "Error did not indicate URL parsing failed `{}`",
            error
        );
    }
}
