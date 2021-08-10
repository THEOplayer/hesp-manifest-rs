use std::time::Duration;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct ScaledValue {
    pub value: i64,
    #[serde(default = "ScaledValue::default_scale")]
    pub scale: u64,
}

impl ScaledValue {
    fn default_scale() -> u64 {
        1
    }

    pub fn new(value: i64) -> ScaledValue {
        ScaledValue { value, scale: 1 }
    }

    pub fn into_duration_secs(self) -> Duration {
        Duration::from_secs_f64(self.into())
    }
}

impl From<ScaledValue> for f64 {
    fn from(value: ScaledValue) -> Self {
        let ScaledValue{value, scale} = value;
        value as f64 / scale as f64
    }
}

impl Default for ScaledValue {
    fn default() -> Self {
        Self {
            value: 0,
            scale: Self::default_scale(),
        }
    }
}