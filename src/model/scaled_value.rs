use serde::{Deserialize, Serialize};

use crate::util::{check_js_safety_unsigned, Int};
use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default)]
pub struct ScaledValue {
    #[serde(deserialize_with = "Int::deserialize_i64")]
    pub value: i64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    pub scale: Scale,
}

impl ScaledValue {
    pub fn is_zero(&self) -> bool {
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
            Ok(Scale(value))
        }
    }
}

impl From<Scale> for u64 {
    fn from(scale: Scale) -> Self {
        scale.0
    }
}

impl Scale {
    pub fn as_f64(self) -> f64 {
        self.0 as f64
    }

    /// Returns whether this is the default scale.
    /// See [default](Scale::default) for more information.
    pub fn is_default(&self) -> bool {
        self == &Scale::default()
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

impl From<ScaledValue> for f64 {
    fn from(input: ScaledValue) -> Self {
        input.value as f64 / input.scale.as_f64()
    }
}
