use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::model::audio::data::AudioSwitchingSetData;
use crate::*;

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<AudioSwitchingSetData>,
    pub base_url: Option<String>,
    pub current_time: Option<ScaledValue>,
    #[serde(default)]
    pub events: Vec<PresentationEvent>,
    #[serde(default)]
    pub metadata: Vec<MetadataSwitchingSetData>,
    #[serde(default)]
    pub video: Vec<VideoSwitchingSetData>,
    pub transmission: PresentationTransmission,
}

impl From<Presentation> for PresentationData {
    fn from(input: Presentation) -> Self {
        Self {
            id: input.id,
            time_bounds: input.time_bounds,
            audio: input.audio.into_iter().map(From::from).collect(),
            base_url: None,
            current_time: input.current_time,
            events: input.events.into_iter().map(From::from).collect(),
            metadata: input.metadata.into_iter().map(From::from).collect(),
            video: input.video.into_iter().map(From::from).collect(),
            transmission: input.transmission,
        }
    }
}
