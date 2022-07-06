use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TimeSource {
    pub scheme: Url,
    pub value: String,
}

#[cfg(test)]
mod tests {
    use anyhow::Result;

    use super::*;

    #[test]
    fn serialize_time_source() -> Result<()> {
        let source = TimeSource {
            scheme: Url::parse("urn:mpeg:dash:utc:ntp:2014")?,
            value: "https://xxx".to_string(),
        };
        let json = serde_json::to_string(&source)?;
        assert_eq!(
            json,
            r#"{"scheme":"urn:mpeg:dash:utc:ntp:2014","value":"https://xxx"}"#
        );
        Ok(())
    }

    #[test]
    fn deserialize_time_source() -> Result<()> {
        let data = r#"
        {
            "scheme": "urn:mpeg:dash:utc:ntp:2014",
            "value": "https://xxx"
        }"#;
        let TimeSource { scheme, value } = serde_json::from_str(data)?;

        assert_eq!(scheme, Url::parse("urn:mpeg:dash:utc:ntp:2014")?);
        assert_eq!(value, "https://xxx");
        Ok(())
    }

    #[test]
    fn invalid_scheme_in_time_source() {
        let data = r#"
        {
            "scheme": "John Doe",
            "value": "https://xxx"
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
}
