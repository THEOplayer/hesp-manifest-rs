use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ScaledValue {
    pub value: i64,
    #[serde(default)]
    pub scale: Scale,
}

#[derive(Deserialize, Debug, Serialize, Clone, Copy, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Scale(u64);

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

    pub fn into_duration_secs(self) -> Duration {
        Duration::from_secs_f64(self.into())
    }
}

impl From<ScaledValue> for f64 {
    fn from(value: ScaledValue) -> Self {
        let ScaledValue { value, scale } = value;
        value as f64 / scale.0 as f64
    }
}

impl Default for ScaledValue {
    fn default() -> Self {
        Self {
            value: 0,
            scale: Scale::default(),
        }
    }
}
