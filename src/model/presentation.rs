use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Presentation {
    id: String,
    time_bounds: TimeBounds,
    #[serde(default)]
    audio: EntityVec<AudioSwitchingSet>,
    base_url: Option<RelativeBaseUrl>,
    current_time: Option<ScaledValue>,
    #[serde(default)]
    events: EntityVec<PresentationEvent>,
    #[serde(default)]
    metadata: EntityVec<MetadataSwitchingSet>,
    #[serde(default)]
    video: EntityVec<VideoSwitchingSet>,
}

impl Presentation {
    pub fn audio(&self) -> &[AudioSwitchingSet] { &self.audio }
    pub fn metadata(&self) -> &[MetadataSwitchingSet] { &self.metadata }
    pub fn video(&self) -> &[VideoSwitchingSet] { &self.video }
    pub fn base_url(&self) -> &Option<RelativeBaseUrl> { &self.base_url }
}

impl Presentation {
    pub(crate) fn validate_active(&self) -> Result<()> {
        if self.current_time.is_none() {
            Err(Error::MissingCurrentTime(self.id.to_owned()))
        } else {
            Ok(())
        }
    }
}

impl Entity for Presentation {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}
