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
    pub fn transmission(&self) -> &PresentationTransmission { &self.transmission }
    pub fn video_switching_set(&self, switching_set_id: &str) -> Option<&VideoSwitchingSet> {
        self.video.get(switching_set_id)
    }
    pub fn audio_switching_set(&self, switching_set_id: &str) -> Option<&AudioSwitchingSet> {
        self.audio.get(switching_set_id)
    }
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

    pub fn video_tracks(&self) -> impl Iterator<Item=(TrackPath, &VideoTrack)> {
        self.tracks(&self.video)
    }

    pub fn audio_tracks(&self) -> impl Iterator<Item=(TrackPath, &AudioTrack)> {
        self.tracks(&self.audio)
    }

    fn tracks<'a, S>(&'a self, selection_set: &'a [S]) -> impl Iterator<Item=(TrackPath, &'a S::MediaTrack)>
        where S: MediaSwitchingSet
    {
        let presentation_id = self.id();
        selection_set.iter().flat_map(move |switching_set| {
            let switching_set_id = switching_set.id();
            switching_set.tracks().iter().map(move |track| {
                let path = TrackPath::new(
                    presentation_id.to_owned(),
                    S::MEDIA_TYPE,
                    switching_set_id.to_owned(),
                    track.id().to_owned(),
                );
                (path, track)
            })
        })
    }

    pub fn video_tracks_mut(&mut self) -> impl Iterator<Item=&mut VideoTrack> {
        self.video.iter_mut().flat_map(|set| set.tracks_mut())
    }

    pub fn audio_tracks_mut(&mut self) -> impl Iterator<Item=&mut AudioTrack> {
        self.audio.iter_mut().flat_map(|set| set.tracks_mut())
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

    pub fn set_unicast(&mut self) {
        self.transmission = PresentationTransmission::Unicast;
        for track in self.video_tracks_mut() {
            track.transmission = TrackTransmission::Unicast;
        }
        for track in self.audio_tracks_mut() {
            track.transmission = TrackTransmission::Unicast;
        }
    }

    pub fn into_multicast<F>(self, meta: PresentationMulticastMetadata, mut toi_provider: F) -> Self
        where F: FnMut(TrackPath) -> TransferObjectIdentifierLimits
    {
        let mut result = self;
        let id = result.id.clone();
        result.transmission = PresentationTransmission::Multicast(meta);
        for set in &mut result.video[..] {
            let set_id = set.id().to_owned();
            for track in set.tracks_mut() {
                let path = TrackPath::new(
                    id.clone(),
                    MediaType::Video,
                    set_id.clone(),
                    track.id().to_owned(),
                );
                track.transmission = TrackTransmission::Multicast { toi_limits: toi_provider(path) }
            }
        }
        for set in &mut result.audio[..] {
            let set_id = set.id().to_owned();
            for track in set.tracks_mut() {
                let path = TrackPath::new(
                    id.clone(),
                    MediaType::Video,
                    set_id.clone(),
                    track.id().to_owned(),
                );
                track.transmission = TrackTransmission::Multicast { toi_limits: toi_provider(path) }
            }
        }
        result
    }

}

impl Entity for Presentation {
    type Id = str;
    fn id(&self) -> &str { &self.id }
}

impl Validate for Presentation {
    fn validate(&self) -> Result<()> {
        for (_, track) in self.video_tracks() { self.validate_track(track)? }
        for (_, track) in self.audio_tracks() { self.validate_track(track)? }
        Ok(())
    }
}