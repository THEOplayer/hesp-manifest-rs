use url::Url;

pub use data::PresentationData;
pub use event::*;
pub use multicast::*;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    AudioSwitchingSet, AudioTrack, Error, MetadataSwitchingSet, MetadataTrack, Result, ScaledValue,
    SwitchingSet, TimeBounds, Track, TrackTransmission, TrackUid, TransferObjectIdentifierLimits,
    TransmissionType, VideoSwitchingSet, VideoTrack,
};

mod data;
mod event;
mod multicast;

#[derive(Clone, Debug)]
pub struct Presentation {
    id: String,
    time_bounds: TimeBounds,
    audio: EntityMap<AudioSwitchingSet>,
    current_time: Option<ScaledValue>,
    events: EntityMap<PresentationEvent>,
    metadata: EntityMap<MetadataSwitchingSet>,
    video: EntityMap<VideoSwitchingSet>,
    transmission: PresentationTransmission,
}

impl Presentation {
    pub fn new(manifest_url: &Url, data: PresentationData) -> Result<Self> {
        let id = data.id;
        let base_url = data.base_url.resolve(manifest_url)?;
        let audio = data
            .audio
            .into_iter()
            .map(|a| AudioSwitchingSet::new(&id, &base_url, a))
            .into_entities()?;
        let metadata = data
            .metadata
            .into_iter()
            .map(|m| MetadataSwitchingSet::new(&id, &base_url, m))
            .into_entities()?;
        let video = data
            .video
            .into_iter()
            .map(|v| VideoSwitchingSet::new(&id, &base_url, v))
            .into_entities()?;
        let result = Self {
            id,
            time_bounds: data.time_bounds,
            audio,
            current_time: data.current_time,
            events: data.events.into_iter().map(Ok).into_entities()?,
            metadata,
            video,
            transmission: data.transmission,
        };
        result.validate_tracks()?;
        Ok(result)
    }

    pub fn audio(&self) -> EntityIter<AudioSwitchingSet> {
        self.audio.iter()
    }
    pub fn audio_mut(&mut self) -> EntityIterMut<AudioSwitchingSet> {
        self.audio.iter_mut()
    }
    pub fn metadata(&self) -> EntityIter<MetadataSwitchingSet> {
        self.metadata.iter()
    }
    pub fn video(&self) -> EntityIter<VideoSwitchingSet> {
        self.video.iter()
    }
    pub fn video_mut(&mut self) -> EntityIterMut<VideoSwitchingSet> {
        self.video.iter_mut()
    }
    pub const fn transmission(&self) -> &PresentationTransmission {
        &self.transmission
    }
    pub fn video_switching_set(&self, switching_set_id: &str) -> Option<&VideoSwitchingSet> {
        self.video.get(switching_set_id)
    }
    pub fn audio_switching_set(&self, switching_set_id: &str) -> Option<&AudioSwitchingSet> {
        self.audio.get(switching_set_id)
    }
    pub fn is_multicast(&self) -> bool {
        return self.transmission().get_type() == TransmissionType::Multicast;
    }
}

impl Presentation {
    pub(super) fn validate_active(&self) -> Result<()> {
        if self.current_time.is_none() {
            return Err(Error::MissingCurrentTime(self.id.clone()));
        }
        for set in &self.video {
            set.validate_active()?;
        }
        for set in &self.audio {
            set.validate_active()?;
        }
        Ok(())
    }
    pub(super) fn ensure_unicast(&self) -> Result<()> {
        match self.transmission {
            PresentationTransmission::Unicast => Ok(()),
            PresentationTransmission::Multicast(_) => {
                Err(Error::InvalidUnicastPresentation(self.id.clone()))
            }
        }
    }

    pub fn video_tracks(&self) -> impl Iterator<Item = &VideoTrack> {
        self.video().flat_map(|set| set.tracks())
    }

    pub fn audio_tracks(&self) -> impl Iterator<Item = &AudioTrack> {
        self.audio().flat_map(|set| set.tracks())
    }

    pub fn metadata_tracks(&self) -> impl Iterator<Item = &MetadataTrack> {
        self.metadata().flat_map(|set| set.tracks())
    }

    pub fn video_tracks_mut(&mut self) -> impl Iterator<Item = &mut VideoTrack> {
        self.video.iter_mut().flat_map(|set| set.tracks_mut())
    }

    pub fn audio_tracks_mut(&mut self) -> impl Iterator<Item = &mut AudioTrack> {
        self.audio.iter_mut().flat_map(|set| set.tracks_mut())
    }

    fn validate_tracks(&self) -> Result<()> {
        for track in self.video_tracks() {
            self.validate_track(track)?;
        }
        for track in self.audio_tracks() {
            self.validate_track(track)?;
        }
        Ok(())
    }

    fn validate_track<T: Track>(&self, track: &T) -> Result<()> {
        let transmission = self.transmission.get_type();
        if track.transmission().get_type() == transmission {
            Ok(())
        } else {
            Err(Error::InvalidTrackTransmission {
                presentation: self.id.clone(),
                track: track.id().to_owned(),
                transmission,
            })
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
    where
        F: FnMut(&TrackUid) -> TransferObjectIdentifierLimits,
    {
        let mut result = self;
        result.transmission = PresentationTransmission::Multicast(meta);
        for set in result.video_mut() {
            for track in set.tracks_mut() {
                track.transmission = TrackTransmission::Multicast {
                    toi_limits: toi_provider(track.uid()),
                }
            }
        }
        for set in result.audio_mut() {
            for track in set.tracks_mut() {
                track.transmission = TrackTransmission::Multicast {
                    toi_limits: toi_provider(track.uid()),
                }
            }
        }
        result
    }
}

impl Entity for Presentation {
    fn id(&self) -> &str {
        &self.id
    }
}
