use gcd::Gcd;
use serde::{Deserialize, Serialize};
use std::ops::{Div, Mul};

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
