use serde::{Deserialize, Serialize};

use crate::{Number, Scale};

#[derive(Deserialize, Debug, Serialize, Clone, Eq, PartialEq)]
pub struct Resolution {
    width: Number,
    height: Number,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    sar_width: Scale,
    #[serde(default, skip_serializing_if = "Scale::is_default")]
    sar_height: Scale,
}
