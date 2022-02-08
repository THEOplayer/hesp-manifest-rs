use gcd::Gcd;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::num::TryFromIntError;
use std::ops::{Div, Mul, Sub};

use crate::util::UInt;
use crate::Scale;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq)]
pub struct UnsignedScaledValue {
    #[serde(deserialize_with = "UInt::deserialize_u64")]
    pub value: u64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl UnsignedScaledValue {
    pub fn new(value: u64, scale: Scale) -> Self {
        Self { value, scale }
    }

    pub fn floor(self) -> u64 {
        self.value / u64::from(self.scale)
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
        Self::new(value, Scale::default())
    }
}

impl Mul<UnsignedScaledValue> for UnsignedScaledValue {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn mul(self, other: Self) -> Self {
        let value = u128::from(self.value) * u128::from(other.value);
        let scale = u128::from(u64::from(self.scale)) * u128::from(u64::from(other.scale));
        from_u128(value, scale).unwrap_or_else(|_| panic!("attempt to multiply with overflow"))
    }
}

impl Div<UnsignedScaledValue> for UnsignedScaledValue {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn div(self, other: Self) -> Self {
        let value = u128::from(self.value) * u128::from(u64::from(other.scale));
        let scale = u128::from(u64::from(self.scale)) * u128::from(other.value);
        from_u128(value, scale).unwrap_or_else(|_| panic!("attempt to divide with overflow"))
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

impl Sub<UnsignedScaledValue> for UnsignedScaledValue {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn sub(self, other: Self) -> Self {
        let scale_a = u128::from(u64::from(self.scale));
        let scale_b = u128::from(u64::from(other.scale));
        let left = u128::from(self.value) * scale_b;
        let right = u128::from(other.value) * scale_a;
        let value = left - right;
        let scale = scale_a * scale_b;
        from_u128(value, scale).unwrap_or_else(|_| panic!("attempt to subtract with overflow"))
    }
}

/// assumes scale is not zero
fn from_u128(
    value: u128,
    scale: u128,
) -> std::result::Result<UnsignedScaledValue, TryFromIntError> {
    let gcd = value.gcd(scale);
    Ok(UnsignedScaledValue::new(
        u64::try_from(value / gcd)?,
        u64::try_from(scale / gcd)?.try_into().unwrap(),
    ))
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
        assert_eq!(a * b, c);
    }

    #[test]
    fn div() {
        let a = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let b = UnsignedScaledValue::new(4, Scale::try_from(5u64).unwrap());
        let c = UnsignedScaledValue::new(5, Scale::try_from(6u64).unwrap());
        assert_eq!(a / b, c);
    }

    #[test]
    fn sub() {
        let a = UnsignedScaledValue::new(5, Scale::try_from(4u64).unwrap());
        let b = UnsignedScaledValue::new(2, Scale::try_from(3u64).unwrap());
        let c = UnsignedScaledValue::new(7, Scale::try_from(12u64).unwrap());
        assert_eq!(a - b, c);
    }

    #[test]
    fn floor() {
        let a = UnsignedScaledValue::new(5, Scale::try_from(4u64).unwrap());
        assert_eq!(a.floor(), 1);
    }
}
