use serde::Deserialize;
use url::Url;
use uuid::Uuid;

#[derive(Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub struct TimeSource {
    pub scheme: Uuid,
    pub url: Url,
}

impl From<TimeSource> for crate::TimeSource {
    fn from(input: TimeSource) -> Self {
        Self {
            scheme: Url::parse(&input.scheme.to_urn().to_string()).unwrap(),
            url: input.url,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
