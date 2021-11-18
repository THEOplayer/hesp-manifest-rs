use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::util::Validate;
use crate::*;

//TODO remove this macros as this is the only one still used
validate_on_deserialize!(TimeBounds);
#[skip_serializing_none]
#[derive(Deserialize, Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct TimeBounds {
    start_time: Option<u64>,
    end_time: Option<u64>,
    #[serde(default = "TimeBounds::default_scale")]
    scale: u64,
}

impl TimeBounds {
    const DEFAULT_SCALE: u64 = 1;
    pub fn default_scale() -> u64 {
        Self::DEFAULT_SCALE
    }

    pub fn duration(self) -> Option<ScaledValue> {
        Some(ScaledValue {
            value: (self.end_time? - self.start_time?) as i64,
            scale: self.scale,
        })
    }
}

impl Validate for TimeBounds {
    fn validate(&self) -> Result<()> {
        if self.start_time.is_none() && self.end_time.is_none() {
            return Err(Error::EmptyTimeBounds);
        }
        if let TimeBounds {
            start_time: Some(start),
            end_time: Some(end),
            ..
        } = *self
        {
            if start >= end {
                return Err(Error::ReverseTimeBounds { start, end });
            }
        }
        Ok(())
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
