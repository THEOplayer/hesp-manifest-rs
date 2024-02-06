use std::fmt;

use serde::{Deserialize, Serialize};

use crate::util::{try_convert_i64_to_float, try_convert_u64_to_float, Int};
use crate::{Error, Result, Scale};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq)]
pub struct ScaledValue {
    #[serde(deserialize_with = "Int::deserialize_i64")]
    pub value: i64,
    #[serde(default, skip_serializing_if = "Scale::is_one")]
    pub scale: Scale,
}

impl ScaledValue {
    #[must_use]
    pub const fn new(value: i64, scale: Scale) -> Self {
        Self { value, scale }
    }
}

impl PartialEq for ScaledValue {
    fn eq(&self, other: &Self) -> bool {
        let left = i128::from(self.value) * i128::from(u64::from(other.scale));
        let right = i128::from(other.value) * i128::from(u64::from(self.scale));
        left == right
    }
}

impl From<i64> for ScaledValue {
    fn from(value: i64) -> Self {
        Self::new(value, Scale::ONE)
    }
}

impl TryFrom<ScaledValue> for f64 {
    type Error = Error;

    fn try_from(input: ScaledValue) -> Result<Self> {
        let value = try_convert_i64_to_float(input.value)?;
        let scale = try_convert_u64_to_float(input.scale.into())?;
        Ok(value / scale)
    }
}

impl fmt::Display for ScaledValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.scale == Scale::ONE {
            self.value.fmt(f)
        } else {
            write!(f, "{}/{}", self.value, self.scale)
        }
    }
}
