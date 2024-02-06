use std::fmt;

use serde::{Deserialize, Serialize};

pub use scaled_value::ScaledValue;
pub use unsigned_scaled_value::UnsignedScaledValue;

use crate::util::check_js_safety_unsigned;
use crate::{Error, Result};

mod scaled_value;
mod unsigned_scaled_value;

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

impl From<Scale> for u128 {
    fn from(scale: Scale) -> Self {
        u64::from(scale).into()
    }
}

impl Scale {
    pub const ONE: Self = Self(1);
    #[must_use]
    pub fn is_one(&self) -> bool {
        *self == Self::ONE
    }
}

impl Default for Scale {
    /// The default scale is 1 because 1 is the multiplicative identity.
    /// (multiplying or dividing by 1 is a no-op.)
    ///
    /// This is also the default value for the HESP Manifest JSON specification `ScaledValue` scale
    /// and therefore can be omitted from the JSON.
    fn default() -> Self {
        Self::ONE
    }
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
