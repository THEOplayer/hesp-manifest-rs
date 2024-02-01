use serde::Deserialize;

use crate::data::{v1_0_0, v1_1_0, v2_0_0};
use crate::util::Uri;
use crate::{Error, Result};
use crate::{PresentationEvent, TimeBounds, UnsignedScaledValue};

#[derive(Clone, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<v1_1_0::AudioSwitchingSetData>,
    pub base_url: Option<Uri>,
    pub current_time: Option<UnsignedScaledValue>,
    #[serde(default)]
    pub events: Vec<PresentationEvent>,
    #[serde(default)]
    pub metadata: Vec<v1_0_0::MetadataSwitchingSetData>,
    #[serde(default)]
    pub video: Vec<v1_0_0::VideoSwitchingSetData>,
}

impl PresentationData {
    #[allow(clippy::option_if_let_else)]
    pub fn validate_legacy_active(&self) -> Result<()> {
        if let Some(track) = self.audio.iter().find_map(|set| {
            set.tracks
                .iter()
                .find(|track| track.active_sequence_number.is_none())
        }) {
            Err(Error::MissingActiveSequenceNumber(track.id.clone()))
        } else if let Some(track) = self.video.iter().find_map(|set| {
            set.tracks
                .iter()
                .find(|track| track.active_sequence_number.is_none())
        }) {
            Err(Error::MissingActiveSequenceNumber(track.id.clone()))
        } else {
            Ok(())
        }
    }
}

impl From<PresentationData> for v2_0_0::PresentationData {
    fn from(input: PresentationData) -> Self {
        Self {
            id: input.id,
            time_bounds: input.time_bounds,
            audio: input
                .audio
                .into_iter()
                .map(v2_0_0::AudioSwitchingSetData::from)
                .collect(),
            base_url: input.base_url,
            events: input.events,
            metadata: input
                .metadata
                .into_iter()
                .map(v2_0_0::MetadataSwitchingSetData::from)
                .collect(),
            video: input
                .video
                .into_iter()
                .map(v2_0_0::VideoSwitchingSetData::from)
                .collect(),
        }
    }
}
