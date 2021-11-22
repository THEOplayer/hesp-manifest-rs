use crate::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ScaledValue {
    pub value: i64,
    #[serde(default)]
    pub scale: Scale,
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
