use std::convert::TryFrom;
use std::ops::Deref;
use std::str::FromStr;

use chrono::{FixedOffset, ParseError, ParseResult};
use serde::{Deserialize, Serialize};

type Inner = chrono::DateTime<FixedOffset>;

#[derive(PartialEq, Deserialize, Serialize, Clone, Debug, Eq, Hash)]
#[serde(try_from = "String", into = "String")]
pub struct DateTime(Inner);

impl FromStr for DateTime {
    type Err = ParseError;
    fn from_str(s: &str) -> ParseResult<Self> {
        chrono::DateTime::parse_from_rfc3339(s).map(DateTime)
    }
}

impl TryFrom<String> for DateTime {
    type Error = ParseError;
    fn try_from(value: String) -> ParseResult<Self> {
        value.parse()
    }
}

impl Deref for DateTime {
    type Target = Inner;
    fn deref(&self) -> &Inner {
        &self.0
    }
}

impl From<DateTime> for String {
    fn from(value: DateTime) -> Self {
        (*value).to_rfc3339()
    }
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use chrono::{FixedOffset, TimeZone};

    use super::*;

    #[test]
    fn serialize_date_time() -> Result<()> {
        let input = "\"2021-03-31T08:00:00.000Z\"";

        let date_time: DateTime = serde_json::from_str(input)?;

        assert_eq!(
            *date_time,
            FixedOffset::east(0)
                .ymd(2021, 3, 31)
                .and_hms_micro(8, 0, 0, 0)
        );
        Ok(())
    }

    #[test]
    fn invalid_date_time() {
        let input = "\"2021--03-31T08:00:00.000Z\"";

        let result = serde_json::from_str::<DateTime>(input);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "input contains invalid characters");
    }
}
