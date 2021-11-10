use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

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
