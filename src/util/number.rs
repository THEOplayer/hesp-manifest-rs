use log::warn;
use serde::{Deserialize, Deserializer, Serialize};

use crate::{Error, Result};

const MAX_SAFE_INTEGER: i64 = 9_007_199_254_740_991;
const MIN_SAFE_INTEGER: i64 = -9_007_199_254_740_991;

#[derive(
    Copy, Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default,
)]
#[serde(from = "u64")]
pub struct UInt(u64);

#[derive(
    Copy, Clone, Serialize, Deserialize, Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Default,
)]
#[serde(from = "i64")]
pub struct Int(i64);

impl UInt {
    pub fn deserialize_u64<'de, D>(deserializer: D) -> std::result::Result<u64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: u64 = Deserialize::deserialize(deserializer)?;
        check_js_safety_unsigned(value);
        Ok(value)
    }
}

impl Int {
    pub fn deserialize_i64<'de, D>(deserializer: D) -> std::result::Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value: i64 = Deserialize::deserialize(deserializer)?;
        check_js_safety(value);
        Ok(value)
    }
}

impl From<u64> for UInt {
    fn from(value: u64) -> Self {
        check_js_safety_unsigned(value);
        Self(value)
    }
}

impl From<i64> for Int {
    fn from(value: i64) -> Self {
        check_js_safety(value);
        Self(value)
    }
}

impl From<UInt> for u64 {
    fn from(value: UInt) -> Self {
        value.0
    }
}

impl From<Int> for i64 {
    fn from(value: Int) -> Self {
        value.0
    }
}

pub fn check_js_safety(number: i64) {
    if number > MAX_SAFE_INTEGER {
        warn!(
            "Number {} is too large to fit in a JavaScript Number",
            number
        );
    } else if number < MIN_SAFE_INTEGER {
        warn!(
            "Number {} is too small to fit in a JavaScript Number",
            number
        );
    }
}

pub fn check_js_safety_unsigned(number: u64) {
    if number > MAX_SAFE_INTEGER as u64 {
        warn!(
            "Number {} is too large to fit in a JavaScript Number",
            number
        );
    }
}

#[allow(clippy::cast_precision_loss, clippy::cast_possible_truncation)]
pub fn try_convert_i64_to_float(number: i64) -> Result<f64> {
    let result = number as f64;
    if result as i64 == number {
        Ok(result)
    } else {
        Err(Error::FloatOverflow(number.to_string()))
    }
}

#[allow(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss
)]
pub fn try_convert_u64_to_float(number: u64) -> Result<f64> {
    let result = number as f64;
    if result as u64 == number {
        Ok(result)
    } else {
        Err(Error::FloatOverflow(number.to_string()))
    }
}
