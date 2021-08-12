use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use crate::*;

validate_on_deserialize!(Presentation);
#[skip_serializing_none]
#[derive(Clone, Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", remote = "Self")]
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
    transmission: PresentationTransmission,
}



impl Presentation {
    pub fn audio(&self) -> &[AudioSwitchingSet] { &self.audio }
    pub fn metadata(&self) -> &[MetadataSwitchingSet] { &self.metadata }
    pub fn video(&self) -> &[VideoSwitchingSet] { &self.video }
    pub fn base_url(&self) -> &Option<RelativeBaseUrl> { &self.base_url }
}

impl Presentation {
    pub(super) fn validate_active(&self) -> Result<()> {
        if self.current_time.is_none() {
            Err(Error::MissingCurrentTime(self.id.to_owned()))
        } else {
            Ok(())
        }
    }
    pub(super) fn ensure_unicast(&self) -> Result<()> {
        match self.transmission {
            PresentationTransmission::Unicast => Ok(()),
            _ => Err(Error::InvalidUnicastPresentation(self.id.to_owned())),
        }
    }

    pub fn video_tracks(&self) -> impl Iterator<Item=&VideoTrack> {
        self.video.iter().flat_map(|x| x.tracks())
    }

    pub fn audio_tracks(&self) -> impl Iterator<Item=&AudioTrack> {
        self.audio.iter().flat_map(|x| x.tracks())
    }

    fn validate_track<T: MediaTrack>(&self, track: &T) -> Result<()> {
        let transmission = self.transmission.get_type();
        if track.transmission().get_type() != transmission {
            Err(Error::InvalidTrackTransmission {
                presentation: self.id.to_owned(),
                track: track.id().to_owned(),
                transmission,
            })
        } else {
            Ok(())
        }
    }
}

impl Entity for Presentation {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}

impl Validate for Presentation {
    fn validate(&self) -> Result<()> {
        for track in self.video_tracks() { self.validate_track(track)? }
        for track in self.audio_tracks() { self.validate_track(track)? }
        Ok(())
    }
}


