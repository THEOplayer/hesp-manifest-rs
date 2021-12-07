use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::util::UInt;
use crate::Scale;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ScaledDuration {
    #[serde(deserialize_with = "UInt::deserialize_u64")]
    pub value: u64, // seconds * scale
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl ScaledDuration {
    pub fn is_zero(&self) -> bool {
        self.value == 0
    }
}

const NANOS_PER_SEC: u32 = 1_000_000_000;

impl From<ScaledDuration> for Duration {
    #[allow(clippy::cast_lossless, clippy::cast_possible_truncation)]
    fn from(duration: ScaledDuration) -> Self {
        let nanos =
            duration.value as u128 * NANOS_PER_SEC as u128 / u64::from(duration.scale) as u128;
        let secs = (nanos / (NANOS_PER_SEC as u128)) as u64;
        let nanos = (nanos % (NANOS_PER_SEC as u128)) as u32;
        Self::new(secs, nanos)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaled_duration_is_lossless_and_does_not_truncate() {
        let duration = ScaledDuration {
            value: u64::MAX,
            scale: Scale::default(),
        };
        let duration: Duration = duration.into();
        assert_eq!(duration.as_secs(), u64::MAX);
        assert_eq!(duration.subsec_nanos(), 0);
    }
}
