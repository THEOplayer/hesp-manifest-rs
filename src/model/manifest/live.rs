use serde::{Deserialize, Serialize};

use crate::{ScaledValue, TimeSource};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveStream {
    pub availability_duration: ScaledValue,
    pub active_presentation: String,
    pub time_source: Option<TimeSource>,
}
