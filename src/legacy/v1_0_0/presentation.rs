use crate::legacy::{v1_0_0, v1_1_0};
use crate::util::Uri;
use crate::{
    MetadataSwitchingSetData, PresentationEvent, TimeBounds, UnsignedScaledValue,
    VideoSwitchingSetData,
};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<v1_0_0::AudioSwitchingSetData>,
    pub base_url: Option<Uri>,
    pub current_time: Option<UnsignedScaledValue>,
    #[serde(default)]
    pub events: Vec<PresentationEvent>,
    #[serde(default)]
    pub metadata: Vec<MetadataSwitchingSetData>,
    #[serde(default)]
    pub video: Vec<VideoSwitchingSetData>,
}

impl From<PresentationData> for v1_1_0::PresentationData {
    fn from(input: PresentationData) -> Self {
        Self {
            id: input.id,
            time_bounds: input.time_bounds,
            audio: input
                .audio
                .into_iter()
                .map(crate::AudioSwitchingSetData::from)
                .collect(),
            current_time: input.current_time,
            base_url: input.base_url,
            events: input.events,
            metadata: input.metadata,
            video: input.video,
        }
    }
}
