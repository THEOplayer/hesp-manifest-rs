use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::util::RelativeUrl;
use crate::{
    AudioSwitchingSetData, MetadataSwitchingSetData, Presentation, PresentationEvent,
    PresentationTransmission, ScaledValue, TimeBounds, VideoSwitchingSetData,
};

#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PresentationData {
    pub id: String,
    pub time_bounds: TimeBounds,
    #[serde(default)]
    pub audio: Vec<AudioSwitchingSetData>,
    pub base_url: RelativeUrl,
    pub current_time: Option<ScaledValue>,
    #[serde(default)]
    pub events: Vec<PresentationEvent>,
    #[serde(default)]
    pub metadata: Vec<MetadataSwitchingSetData>,
    #[serde(default)]
    pub video: Vec<VideoSwitchingSetData>,
    pub transmission: PresentationTransmission,
}

impl PresentationData {
    pub fn new(input: Presentation, location: &Url) -> Self {
        Self {
            id: input.id,
            time_bounds: input.time_bounds,
            audio: input
                .audio
                .into_iter()
                .map(|a| AudioSwitchingSetData::new(a, location))
                .collect(),
            base_url: RelativeUrl::None,
            current_time: input.current_time,
            events: input
                .events
                .into_iter()
                .map(PresentationEvent::from)
                .collect(),
            metadata: input
                .metadata
                .into_iter()
                .map(|m| MetadataSwitchingSetData::new(m, location))
                .collect(),
            video: input
                .video
                .into_iter()
                .map(|v| VideoSwitchingSetData::new(v, location))
                .collect(),
            transmission: input.transmission,
        }
    }

    pub fn normalize(&mut self) {
        for audio in &mut self.audio {
            audio.normalize();
        }
        // for video in &mut self.video {
        //     video.normalize();
        // }
        // for metadata in &mut self.metadata {
        //     metadata.normalize();
        // }
    }
}
