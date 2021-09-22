use serde::{Deserialize, Serialize};

use super::*;

#[derive(Deserialize, Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Resolution {
    width: Number,
    height: Number,
    #[serde(default = "Resolution::default_sar")]
    sar_width: Number,
    #[serde(default = "Resolution::default_sar")]
    sar_height: Number,
}

impl Resolution {
    fn default_sar() -> Number {
        Number::from(1)
    }
}
