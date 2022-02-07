use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{Scale, UnsignedScaledValue};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ScaledDuration(UnsignedScaledValue);
impl ScaledDuration {
    pub fn new(value: u64, scale: Scale) -> Self {
        Self(UnsignedScaledValue::new(value, scale))
    }
}

const NANOS_PER_SEC: u32 = 1_000_000_000;

impl From<ScaledDuration> for Duration {
    #[allow(clippy::cast_lossless, clippy::cast_possible_truncation)]
    fn from(duration: ScaledDuration) -> Self {
        let nanos =
            duration.0.value as u128 * NANOS_PER_SEC as u128 / u64::from(duration.0.scale) as u128;
        let secs = (nanos / (NANOS_PER_SEC as u128)) as u64;
        let nanos = (nanos % (NANOS_PER_SEC as u128)) as u32;
        Self::new(secs, nanos)
    }
}

impl From<ScaledDuration> for UnsignedScaledValue {
    fn from(duration: ScaledDuration) -> Self {
        duration.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaled_duration_is_lossless_and_does_not_truncate() {
        let duration = ScaledDuration::new(u64::MAX, Scale::default());
        let duration: Duration = duration.into();
        assert_eq!(duration.as_secs(), u64::MAX);
        assert_eq!(duration.subsec_nanos(), 0);
    }
}
