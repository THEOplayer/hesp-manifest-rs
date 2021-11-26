use crate::util::RelativeUrl;
use crate::{
    MetadataSwitchingSetData, PresentationEvent, PresentationMulticastMetadata, ScaledValue,
    TimeBounds,
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
    pub video: Vec<super::VideoSwitchingSetData>,
    pub multicast_metadata: Option<PresentationMulticastMetadata>,
}
