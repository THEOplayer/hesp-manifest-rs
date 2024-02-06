use serde::Deserialize;

use crate::data::{v1_0_0, v1_1_0};
use crate::{Error, Result, ScaledDuration};

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

impl LiveStream {
    fn convert(self, presentations: &[v1_1_0::PresentationData]) -> Result<crate::LiveStream> {
        let active_presentation = presentations
            .iter()
            .find(|p| p.id == self.active_presentation)
            .ok_or_else(|| Error::InvalidActivePresentationId(self.active_presentation.clone()))?;
        Ok(crate::LiveStream {
            availability_duration: self.availability_duration,
            time_source: self.time_source.map(crate::TimeSource::from),
            current_time: active_presentation
                .current_time
                .ok_or_else(|| Error::MissingCurrentTime(self.active_presentation.clone()))?,
            active_presentation: self.active_presentation,
        })
    }
}

impl StreamType {
    pub fn convert(self, presentations: &[v1_1_0::PresentationData]) -> Result<crate::StreamType> {
        Ok(match self {
            Self::Live(input) => crate::StreamType::Live(input.convert(presentations)?),
            Self::Vod => crate::StreamType::Vod,
        })
    }
}
