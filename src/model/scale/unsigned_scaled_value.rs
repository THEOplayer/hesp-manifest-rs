use gcd::Gcd;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
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
        let gcd = value.gcd(scale);
        Self::new(
            (value / gcd) as u64,
            ((scale / gcd) as u64).try_into().unwrap(),
        )
    }
}

impl Div<UnsignedScaledValue> for UnsignedScaledValue {
    type Output = Self;

    #[allow(clippy::cast_possible_truncation)]
    fn div(self, other: Self) -> Self {
        let value = u128::from(self.value) * u128::from(u64::from(other.scale));
        let scale = u128::from(u64::from(self.scale)) * u128::from(other.value);
        let gcd = value.gcd(scale);
        Self::new(
            (value / gcd) as u64,
            ((scale / gcd) as u64).try_into().unwrap(),
        )
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
        let gcd = value.gcd(scale);
        Self::new(
            (value / gcd) as u64,
            (u64::try_from(scale / gcd).unwrap()).try_into().unwrap(),
        )
    }
}
