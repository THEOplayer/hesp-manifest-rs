pub use data::PresentationData;
pub use event::*;
pub use multicast::*;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Address, AudioSwitchingSet, AudioTrack, Error, InitializableTrack, MediaType,
    MetadataSwitchingSet, MetadataTrack, Result, ScaledValue, SwitchingSet, TimeBounds, Track,
    TrackTransmission, TrackUid, TransferObjectIdentifierLimits, TransmissionType,
    ValidateSwitchingSet, VideoSwitchingSet, VideoTrack,
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
    pub fn new(manifest_address: &Address, data: PresentationData) -> Result<Self> {
        let id = data.id;
        let address = manifest_address.join(data.base_url)?;
        let audio = data
            .audio
            .into_iter()
            .map(|a| AudioSwitchingSet::new(&id, &address, a))
            .into_entities()?;
        let metadata = data
            .metadata
            .into_iter()
            .map(|m| MetadataSwitchingSet::new(&id, &address, m))
            .into_entities()?;
        let video = data
            .video
            .into_iter()
            .map(|v| VideoSwitchingSet::new(&id, &address, v))
            .into_entities()?;
        let result = Self {
            id,
            time_bounds: data.time_bounds,
            audio,
            current_time: data.current_time,
            events: data.events.into_iter().map(Ok).into_entities()?,
            metadata,
            video,
            transmission: data.multicast_metadata.into(),
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
        self.video().flat_map(VideoSwitchingSet::tracks)
    }

    pub fn audio_tracks(&self) -> impl Iterator<Item = &AudioTrack> {
        self.audio().flat_map(AudioSwitchingSet::tracks)
    }

    pub fn metadata_tracks(&self) -> impl Iterator<Item = &MetadataTrack> {
        self.metadata().flat_map(MetadataSwitchingSet::tracks)
    }

    pub fn video_tracks_mut(&mut self) -> impl Iterator<Item = &mut VideoTrack> {
        self.video
            .iter_mut()
            .flat_map(VideoSwitchingSet::tracks_mut)
    }

    pub fn audio_tracks_mut(&mut self) -> impl Iterator<Item = &mut AudioTrack> {
        self.audio
            .iter_mut()
            .flat_map(AudioSwitchingSet::tracks_mut)
    }

    pub fn metadata_tracks_mut(&mut self) -> impl Iterator<Item = &mut MetadataTrack> {
        self.metadata
            .iter_mut()
            .flat_map(MetadataSwitchingSet::tracks_mut)
    }

    pub fn tracks(&self) -> impl Iterator<Item = &dyn Track> {
        self.audio_tracks()
            .map(|track| track as &dyn Track)
            .chain(self.video_tracks().map(|track| track as &dyn Track))
            .chain(self.metadata_tracks().map(|track| track as &dyn Track))
    }

    pub fn tracks_mut(&mut self) -> impl Iterator<Item = &mut dyn Track> {
        let audio_iter = Self::as_track_mut_iterator(&mut self.audio);
        let video_iter = Self::as_track_mut_iterator(&mut self.video);
        let metadata_iter = Self::as_track_mut_iterator(&mut self.metadata);

        audio_iter.chain(video_iter).chain(metadata_iter)
    }

    fn as_track_mut_iterator<T: SwitchingSet>(
        map: &mut EntityMap<T>,
    ) -> impl Iterator<Item = &mut dyn Track> {
        map.iter_mut()
            .flat_map(SwitchingSet::tracks_mut)
            .map(|track| track as &mut dyn Track)
    }

    pub fn initializable_tracks(&self) -> impl Iterator<Item = &dyn InitializableTrack> {
        self.audio_tracks()
            .map(|track| track as &dyn InitializableTrack)
            .chain(
                self.video_tracks()
                    .map(|track| track as &dyn InitializableTrack),
            )
    }

    pub fn initializable_tracks_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut dyn InitializableTrack> {
        let audio_iter = Self::as_initializable_track_mut_iterator(&mut self.audio);
        let video_iter = Self::as_initializable_track_mut_iterator(&mut self.video);

        audio_iter.chain(video_iter)
    }

    fn as_initializable_track_mut_iterator<'a, T, U>(
        map: &'a mut EntityMap<T>,
    ) -> impl Iterator<Item = &mut dyn InitializableTrack>
    where
        T: SwitchingSet<Track = U>,
        U: InitializableTrack + 'a,
    {
        map.iter_mut()
            .flat_map(SwitchingSet::tracks_mut)
            .map(|track| track as &mut dyn InitializableTrack)
    }

    pub fn track(
        &self,
        media_type: MediaType,
        switching_set_id: &str,
        track_id: &str,
    ) -> Option<&dyn Track> {
        match media_type {
            MediaType::Audio => self
                .audio
                .get(switching_set_id)?
                .track(track_id)
                .map(|track| track as &dyn Track),
            MediaType::Video => self
                .video
                .get(switching_set_id)?
                .track(track_id)
                .map(|track| track as &dyn Track),
            MediaType::Metadata => self
                .metadata
                .get(switching_set_id)?
                .track(track_id)
                .map(|track| track as &dyn Track),
        }
    }

    pub fn track_mut(
        &mut self,
        media_type: MediaType,
        switching_set_id: &str,
        track_id: &str,
    ) -> Option<&mut dyn Track> {
        match media_type {
            MediaType::Audio => self
                .audio
                .get_mut(switching_set_id)?
                .track_mut(track_id)
                .map(|track| track as &mut dyn Track),
            MediaType::Video => self
                .video
                .get_mut(switching_set_id)?
                .track_mut(track_id)
                .map(|track| track as &mut dyn Track),
            MediaType::Metadata => self
                .metadata
                .get_mut(switching_set_id)?
                .track_mut(track_id)
                .map(|track| track as &mut dyn Track),
        }
    }

    pub fn initializable_track(
        &self,
        media_type: MediaType,
        switching_set_id: &str,
        track_id: &str,
    ) -> Option<&dyn InitializableTrack> {
        match media_type {
            MediaType::Audio => self
                .audio
                .get(switching_set_id)?
                .track(track_id)
                .map(|track| track as &dyn InitializableTrack),
            MediaType::Video => self
                .video
                .get(switching_set_id)?
                .track(track_id)
                .map(|track| track as &dyn InitializableTrack),
            MediaType::Metadata => None,
        }
    }

    pub fn initializable_track_mut(
        &mut self,
        media_type: MediaType,
        switching_set_id: &str,
        track_id: &str,
    ) -> Option<&mut dyn InitializableTrack> {
        match media_type {
            MediaType::Audio => self
                .audio
                .get_mut(switching_set_id)?
                .track_mut(track_id)
                .map(|track| track as &mut dyn InitializableTrack),
            MediaType::Video => self
                .video
                .get_mut(switching_set_id)?
                .track_mut(track_id)
                .map(|track| track as &mut dyn InitializableTrack),
            MediaType::Metadata => None,
        }
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
