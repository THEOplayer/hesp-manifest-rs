use std::cmp::Ordering;
use std::fmt;

use gcd::Gcd;
use serde::{Deserialize, Serialize};

use crate::util::UInt;
use crate::Scale;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq)]
pub struct UnsignedScaledValue {
    #[serde(deserialize_with = "UInt::deserialize_u64")]
    pub value: u64,
    #[serde(default, skip_serializing_if = "Scale::is_one")]
    pub scale: Scale,
}

impl UnsignedScaledValue {
    pub const ZERO: Self = Self::new(0, Scale::ONE);

    #[must_use]
    pub const fn new(value: u64, scale: Scale) -> Self {
        Self { value, scale }
    }

    #[must_use]
    pub fn floor(self) -> u64 {
        self.value / u64::from(self.scale)
    }

    #[must_use]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        let scale_a = u128::from(self.scale);
        let scale_b = u128::from(other.scale);
        let left = u128::from(self.value) * scale_b;
        let right = u128::from(other.value) * scale_a;
        let value = left.checked_add(right)?;
        let scale = scale_a * scale_b;
        checked_from_u128(value, scale)
    }

    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        let scale_a = u128::from(self.scale);
        let scale_b = u128::from(other.scale);
        let left = u128::from(self.value) * scale_b;
        let right = u128::from(other.value) * scale_a;
        let value = left.checked_sub(right)?;
        let scale = scale_a * scale_b;
        checked_from_u128(value, scale)
    }

    #[must_use]
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        let value = u128::from(self.value) * u128::from(other.value);
        let scale = u128::from(self.scale) * u128::from(other.scale);
        checked_from_u128(value, scale)
    }

    #[must_use]
    pub fn checked_div(self, other: Self) -> Option<Self> {
        let value = u128::from(self.value) * u128::from(other.scale);
        let scale = u128::from(self.scale) * u128::from(other.value);
        checked_from_u128(value, scale)
    }

    #[allow(clippy::cast_precision_loss)]
    #[must_use]
    pub fn to_secs(self) -> f64 {
        self.value as f64 / self.scale.0 as f64
    }
}

impl PartialEq for UnsignedScaledValue {
    fn eq(&self, other: &Self) -> bool {
        let left = u128::from(self.value) * u128::from(u64::from(other.scale));
        let right = u128::from(other.value) * u128::from(u64::from(self.scale));
        left == right
    }
}

impl From<u64> for UnsignedScaledValue {
    fn from(value: u64) -> Self {
        Self::new(value, Scale::ONE)
    }
}

impl Ord for UnsignedScaledValue {
    fn cmp(&self, other: &Self) -> Ordering {
        let left = u128::from(self.value) * u128::from(u64::from(other.scale));
        let right = u128::from(other.value) * u128::from(u64::from(self.scale));
        left.cmp(&right)
    }
}

impl PartialOrd for UnsignedScaledValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl fmt::Display for UnsignedScaledValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scale == Scale::ONE {
            self.value.fmt(f)
        } else {
            write!(f, "{}/{}", self.value, self.scale)
        }
    }
}

/// assumes scale is not zero
fn checked_from_u128(value: u128, scale: u128) -> Option<UnsignedScaledValue> {
    Some(if value == 0 {
        UnsignedScaledValue::ZERO
    } else {
        let gcd = value.gcd(scale);
        UnsignedScaledValue::new(
            u64::try_from(value / gcd).ok()?,
            u64::try_from(scale / gcd).ok()?.try_into().unwrap(),
        )
    })
}

// tests
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn mul() {
        let a = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let b = UnsignedScaledValue::new(5, Scale::try_from(4u64).unwrap());
        let c = UnsignedScaledValue::new(5, Scale::try_from(6u64).unwrap());
        assert_eq!(a.checked_mul(b), Some(c));
    }

    #[test]
    fn div() {
        let a = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let b = UnsignedScaledValue::new(4, Scale::try_from(5u64).unwrap());
        let c = UnsignedScaledValue::new(5, Scale::try_from(6u64).unwrap());
        assert_eq!(a.checked_div(b), Some(c));
    }

    #[test]
    fn add() {
        let a = UnsignedScaledValue::new(5, Scale::try_from(4u64).unwrap());
        let b = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let c = UnsignedScaledValue::new(23, Scale::try_from(12u64).unwrap());
        assert_eq!(a.checked_add(b), Some(c));
    }

    #[test]
    fn sub() {
        let a = UnsignedScaledValue::new(5, Scale::try_from(4u64).unwrap());
        let b = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let c = UnsignedScaledValue::new(7, Scale::try_from(12u64).unwrap());
        assert_eq!(a.checked_sub(b), Some(c));
    }

    #[test]
    fn floor() {
        let a = UnsignedScaledValue::new(9, Scale::try_from(4u64).unwrap());
        assert_eq!(a.floor(), 2);
    }
}
