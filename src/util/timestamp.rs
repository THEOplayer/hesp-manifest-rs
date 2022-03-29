use chrono::{DateTime, FixedOffset, ParseError, ParseResult};
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::str::FromStr;

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct Timestamp(DateTime<FixedOffset>);

impl FromStr for Timestamp {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<Self> {
        chrono::DateTime::parse_from_rfc3339(s).map(Timestamp)
    }
}

impl TryFrom<String> for Timestamp {
    type Error = ParseError;
    fn try_from(value: String) -> ParseResult<Self> {
        value.parse()
    }
}

impl From<Timestamp> for String {
    fn from(value: Timestamp) -> Self {
        value.0.to_rfc3339()
    }
}

impl From<Timestamp> for DateTime<FixedOffset> {
    fn from(value: Timestamp) -> Self {
        value.0
    }
}

impl From<DateTime<FixedOffset>> for Timestamp {
    fn from(value: DateTime<FixedOffset>) -> Self {
        Self(value)
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::{FixedOffset, TimeZone};

    use super::*;

    #[test]
    fn serialize_timestamp() -> Result<()> {
        let input = "\"2021-03-31T08:00:00.000Z\"";

        let from_str: Timestamp = serde_json::from_str(input)?;
        let from_fixed: Timestamp = FixedOffset::east(0)
            .ymd(2021, 3, 31)
            .and_hms_micro(8, 0, 0, 0)
            .into();

        assert_eq!(from_str, from_fixed);
        Ok(())
    }

    #[test]
    fn invalid_timestamp() {
        let input = "\"2021--03-31T08:00:00.000Z\"";

        let result = serde_json::from_str::<Timestamp>(input);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "input contains invalid characters");
    }
}
