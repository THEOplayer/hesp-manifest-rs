use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

use crate::{Scale, UnsignedScaledValue};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct ScaledDuration(UnsignedScaledValue);

impl ScaledDuration {
    #[must_use]
    pub const fn new(value: u64, scale: Scale) -> Self {
        Self(UnsignedScaledValue::new(value, scale))
    }

    #[must_use]
    pub fn to_secs(self) -> f64 {
        self.0.to_secs()
    }
}

const NANOS_PER_SEC: u128 = 1_000_000_000;

#[allow(clippy::fallible_impl_from)]
impl From<ScaledDuration> for Duration {
    fn from(duration: ScaledDuration) -> Self {
        let nanos =
            u128::from(duration.0.value) * NANOS_PER_SEC / u128::from(u64::from(duration.0.scale));
        let secs = u64::try_from(nanos / NANOS_PER_SEC).unwrap();
        let nanos = u32::try_from(nanos % NANOS_PER_SEC).unwrap();
        Self::new(secs, nanos)
    }
}

impl From<ScaledDuration> for UnsignedScaledValue {
    fn from(duration: ScaledDuration) -> Self {
        duration.0
    }
}

impl fmt::Display for ScaledDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.0.scale == Scale::ONE {
            write!(f, "{}s", self.0.value)
        } else {
            write!(f, "{}s/{}", self.0.value, self.0.scale)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scaled_duration_is_lossless_and_does_not_truncate() {
        let duration = ScaledDuration::new(u64::MAX, Scale::ONE);
        let duration: Duration = duration.into();
        assert_eq!(duration.as_secs(), u64::MAX);
        assert_eq!(duration.subsec_nanos(), 0);
    }
}
