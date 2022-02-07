use serde::{Deserialize, Serialize};

use crate::util::{
    check_js_safety_unsigned, try_convert_i64_to_float, try_convert_u64_to_float, Int, UInt,
};
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq)]
pub struct ScaledValue {
    #[serde(deserialize_with = "Int::deserialize_i64")]
    pub value: i64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq)]
pub struct UnsignedScaledValue {
    #[serde(deserialize_with = "UInt::deserialize_u64")]
    pub value: u64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl ScaledValue {
    pub fn new(value: i64, scale: Scale) -> Self {
        Self { value, scale }
    }
}

impl UnsignedScaledValue {
    pub fn new(value: u64, scale: Scale) -> Self {
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

impl PartialEq for UnsignedScaledValue {
    fn eq(&self, other: &Self) -> bool {
        let left = u128::from(self.value) * u128::from(u64::from(other.scale));
        let right = u128::from(other.value) * u128::from(u64::from(self.scale));
        left == right
    }
}

impl From<i64> for ScaledValue {
    fn from(value: i64) -> Self {
        Self::new(value, Scale::default())
    }
}

impl From<u64> for UnsignedScaledValue {
    fn from(value: u64) -> Self {
        Self::new(value, Scale::default())
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

impl TryFrom<u32> for Scale {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self> {
        if value == 0 {
            Err(Error::NullScale())
        } else {
            Ok(Self(value.into()))
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
