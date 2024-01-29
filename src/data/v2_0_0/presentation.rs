use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::data::{AudioSwitchingSetData, MetadataSwitchingSetData, VideoSwitchingSetData};
use crate::util::{Entity, Uri};
use crate::{Presentation, PresentationEvent, TimeBounds};

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio: Vec<AudioSwitchingSetData>,
    pub base_url: Option<Uri>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub events: Vec<PresentationEvent>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub metadata: Vec<MetadataSwitchingSetData>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub video: Vec<VideoSwitchingSetData>,
}

impl From<Presentation> for PresentationData {
    fn from(input: Presentation) -> Self {
        Self {
            id: input.id().to_string(),
            time_bounds: input.time_bounds,
            audio: input
                .audio
                .into_iter()
                .map(AudioSwitchingSetData::from)
                .collect(),
            base_url: None,
            events: input
                .events
                .into_iter()
                .map(PresentationEvent::from)
                .collect(),
            metadata: input
                .metadata
                .into_iter()
                .map(MetadataSwitchingSetData::from)
                .collect(),
            video: input
                .video
                .into_iter()
                .map(VideoSwitchingSetData::from)
                .collect(),
        }
    }
}

impl PresentationData {
    pub fn normalize(&mut self) {
        for audio in &mut self.audio {
            audio.normalize();
        }
        for video in &mut self.video {
            video.normalize();
        }
        for metadata in &mut self.metadata {
            metadata.normalize();
        }
    }
}
