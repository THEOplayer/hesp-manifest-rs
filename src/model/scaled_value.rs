use serde::{Deserialize, Serialize};

use crate::util::{
    check_js_safety_unsigned, try_convert_i64_to_float, try_convert_u64_to_float, Int,
};
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ScaledValue {
    #[serde(deserialize_with = "Int::deserialize_i64")]
    pub value: i64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl ScaledValue {
    pub const fn is_zero(&self) -> bool {
        self.value == 0
    }
}

impl PartialEq for ScaledValue {
    fn eq(&self, other: &Self) -> bool {
        let left = i128::from(self.value) * i128::from(u64::from(other.scale));
        let right = i128::from(other.value) * i128::from(u64::from(self.scale));
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
            check_js_safety_unsigned(value);
            Ok(Self(value))
        }
    }
}

impl From<Scale> for u64 {
    fn from(scale: Scale) -> Self {
        scale.0
    }
}

impl Scale {
    /// Returns whether this is the default scale.
    /// See [default](Scale::default) for more information.
    pub fn is_default(&self) -> bool {
        self == &Self::default()
    }
}

impl Default for Scale {
    /// The default scale is 1 because 1 is the multiplicative identity.
    /// (multiplying or dividing by 1 is a no-op.)
    ///
    /// This is also the default value for the HESP Manifest JSON specification `ScaledValue` scale
    /// and therefore can be omitted from the JSON.
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

impl TryFrom<ScaledValue> for f64 {
    type Error = Error;

    fn try_from(input: ScaledValue) -> Result<Self> {
        let value = try_convert_i64_to_float(input.value)?;
        let scale = try_convert_u64_to_float(input.scale.into())?;
        Ok(value / scale)
    }
}
