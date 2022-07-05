use crate::util::Uri;
use crate::{AudioSwitchingSetData, MetadataSwitchingSetData, PresentationEvent, TimeBounds, UnsignedScaledValue, VideoSwitchingSetData};
use serde::Deserialize;

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<AudioSwitchingSetData>,
    pub base_url: Option<Uri>,
    pub current_time: Option<UnsignedScaledValue>,
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
            audio: input.audio,
            base_url: input.base_url,
            events: input.events,
            metadata: input.metadata,
            video: input.video,
        }
    }
}
