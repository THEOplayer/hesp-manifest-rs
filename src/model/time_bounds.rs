use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::{Error, Result, Scale, ScaledDuration};

#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase", try_from = "TimeBoundsData")]
pub struct TimeBounds {
    start_time: Option<u64>,
    end_time: Option<u64>,
    scale: Scale,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct TimeBoundsData {
    start_time: Option<u64>,
    end_time: Option<u64>,
    #[serde(default)]
    scale: Scale,
}

impl TimeBounds {
    pub const fn new(start_time: Option<u64>, end_time: Option<u64>, scale: Scale) -> Result<Self> {
        match (start_time, end_time) {
            (None, None) => Err(Error::EmptyTimeBounds),
            (Some(start), Some(end)) if start > end => Err(Error::ReverseTimeBounds { start, end }),
            _ => Ok(Self {
                start_time,
                end_time,
                scale,
            }),
        }
    }

    pub fn duration(self) -> Option<ScaledDuration> {
        Some(ScaledDuration {
            value: self.end_time? - self.start_time?,
            scale: self.scale,
        })
    }

    pub fn start_time(&self) -> Option<u64> {
        self.start_time
    }

    pub fn end_time(&self) -> Option<u64> {
        self.end_time
    }

    pub fn scale(&self) -> Scale {
        self.scale
    }
}

impl TryFrom<TimeBoundsData> for TimeBounds {
    type Error = Error;

    fn try_from(value: TimeBoundsData) -> Result<Self> {
        Self::new(value.start_time, value.end_time, value.scale)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_bounds_needs_start_or_end() {
        let data = "{}";
        let result = serde_json::from_str::<TimeBounds>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("TimeBounds must have a start- or end-time"),
            "Error did not indicate the need for start and end `{}`",
            error
        );
    }

    #[test]
    fn time_bounds_start_needs_to_be_before_end() {
        let data = r#"
            {
                "startTime": 10,
                "endTime": 0
            }"#;
        let result = serde_json::from_str::<TimeBounds>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert_eq!(error, "startTime 10 must be before endTime 0");
    }
}
