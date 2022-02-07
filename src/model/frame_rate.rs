use crate::{Scale, UnsignedScaledValue};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Default, Eq, PartialEq)]
pub struct FrameRate(UnsignedScaledValue);

impl FrameRate {
    pub fn new(value: u64, scale: Scale) -> Self {
        Self(UnsignedScaledValue::new(value, scale))
    }
}
