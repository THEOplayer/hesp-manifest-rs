use serde::{Deserialize, Serialize};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TimeSource {
    scheme: Uuid,
    url: Url,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn serialize_time_source() -> Result<()> {
        let data = r#"
        {
            "scheme": "550e8400-e29b-41d4-a716-446655440000",
            "url": "https://xxx"
        }"#;
        let TimeSource { scheme, url } = serde_json::from_str(data)?;

        assert_eq!(
            scheme,
            Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?
        );
        assert_eq!(url, Url::parse("https://xxx")?);
        Ok(())
    }

    #[test]
    fn invalid_uuid_in_time_source() {
        let data = r#"
        {
            "scheme": "John Doe",
            "url": "https://xxx"
        }"#;
        let result = serde_json::from_str::<TimeSource>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("UUID parsing failed"),
            "Error did not indicate UUID parsing failed `{}`",
            error
        );
    }

    #[test]
    fn invalid_url_in_time_source() {
        let data = r#"
        {
            "scheme": "550e8400-e29b-41d4-a716-446655440000",
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
