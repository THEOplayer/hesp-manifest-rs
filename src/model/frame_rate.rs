use serde::{Deserialize, Serialize};

use crate::{Scale, UnsignedScaledValue};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FrameRate(UnsignedScaledValue);

impl FrameRate {
    #[must_use]
    pub const fn new(value: u64, scale: Scale) -> Self {
        Self(UnsignedScaledValue::new(value, scale))
    }

    #[must_use]
    pub fn to_f64(self) -> f64 {
        self.0.to_f64()
    }
}

impl From<FrameRate> for UnsignedScaledValue {
    fn from(frame_rate: FrameRate) -> Self {
        frame_rate.0
    }
}
