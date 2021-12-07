use serde::{Deserialize, Serialize};

use crate::Scale;

#[derive(Deserialize, Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Resolution {
    width: u64,
    height: u64,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    sar_width: Scale,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    sar_height: Scale,
}
