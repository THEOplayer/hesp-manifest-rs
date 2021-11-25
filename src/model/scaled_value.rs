use serde::{Deserialize, Serialize};

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ScaledValue {
    pub value: i64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl ScaledValue {
    pub fn is_none(&self) -> bool {
        self.value == 0
    }
}

impl PartialEq for ScaledValue {
    fn eq(&self, other: &Self) -> bool {
        let left = i128::from(self.value) * i128::from(other.scale.as_u64());
        let right = i128::from(other.value) * i128::from(self.scale.as_u64());
        left == right
    }
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
#[serde(try_from = "u64")]
pub struct Scale(u64);

impl TryFrom<u64> for Scale {
    type Error = Error;

    fn try_from(value: u64) -> Result<Self> {
        if value == 0 {
            Err(Error::NullScale())
        } else {
            Ok(Scale(value))
        }
    }
}

impl Scale {
    pub fn as_u64(self) -> u64 {
        self.0
    }

    pub fn as_f64(self) -> f64 {
        self.0 as f64
    }

    pub fn is_default(&self) -> bool {
        self.0 == 1
    }
}

impl Default for Scale {
    fn default() -> Self {
        Self(1)
    }
}

impl ScaledValue {
    pub fn new(value: i64) -> Self {
        Self {
            value,
            scale: Scale::default(),
        }
    }
}

impl From<ScaledValue> for f64 {
    fn from(input: ScaledValue) -> Self {
        input.value as f64 / input.scale.as_f64()
    }
}
