use serde::{Deserialize, Serialize};

use crate::{ScaledDuration, TimeSource, UnsignedScaledValue};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveStream {
    pub availability_duration: ScaledDuration,
    pub active_presentation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_source: Option<TimeSource>,
    pub current_time: UnsignedScaledValue,
}

#[allow(clippy::large_enum_variant)]
#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum StreamType {
    Live(LiveStream),
    Vod,
}
