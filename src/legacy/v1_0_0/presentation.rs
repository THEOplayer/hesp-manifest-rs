use crate::util::RelativeUrl;
use crate::{
    MetadataSwitchingSetData, PresentationEvent, ScaledValue, TimeBounds, VideoSwitchingSetData,
};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<super::AudioSwitchingSetData>,
    pub base_url: RelativeUrl,
    pub current_time: Option<ScaledValue>,
    #[serde(default)]
    pub events: Vec<PresentationEvent>,
    #[serde(default)]
    pub metadata: Vec<MetadataSwitchingSetData>,
    #[serde(default)]
    pub video: Vec<VideoSwitchingSetData>,
}

impl From<PresentationData> for crate::PresentationData {
    fn from(input: PresentationData) -> Self {
        Self {
            id: input.id,
            time_bounds: input.time_bounds,
            audio: input
                .audio
                .into_iter()
                .map(crate::AudioSwitchingSetData::from)
                .collect(),
            base_url: input.base_url,
            current_time: input.current_time,
            events: input.events,
            metadata: input.metadata,
            video: input.video,
            multicast_metadata: None,
        }
    }
}
