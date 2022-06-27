use serde::Deserialize;

use crate::legacy::v1_0_0;
use crate::ScaledDuration;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveStream {
    pub availability_duration: ScaledDuration,
    pub active_presentation: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_source: Option<v1_0_0::TimeSource>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum StreamType {
    Live(LiveStream),
    Vod,
}

impl From<LiveStream> for crate::LiveStream {
    fn from(input: LiveStream) -> Self {
        Self {
            availability_duration: input.availability_duration,
            active_presentation: input.active_presentation,
            time_source: input.time_source.map(crate::TimeSource::from),
        }
    }
}

impl From<StreamType> for crate::StreamType {
    fn from(input: StreamType) -> Self {
        match input {
            StreamType::Live(live) => Self::Live(live.into()),
            StreamType::Vod => Self::Vod,
        }
    }
}
