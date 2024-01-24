pub use data::PresentationData;
pub use event::*;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Address, AudioSwitchingSet, AudioTrack, Error, InitializableTrack, MediaType,
    MetadataSwitchingSet, MetadataTrack, Result, SwitchingSet, TimeBounds, Track,
    VideoSwitchingSet, VideoTrack,
};

mod data;
mod event;

#[derive(Clone, Debug)]
pub struct Presentation {
    id: String,
    time_bounds: TimeBounds,
    audio: EntityMap<AudioSwitchingSet>,
    events: EntityMap<PresentationEvent>,
    metadata: EntityMap<MetadataSwitchingSet>,
    video: EntityMap<VideoSwitchingSet>,
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
        Ok(Self {
            id,
            time_bounds: data.time_bounds,
            audio,
            events: data.events.into_iter().map(Ok).into_entities()?,
            metadata,
            video,
        })
    }

    #[must_use]
    pub fn audio(&self) -> EntityIter<AudioSwitchingSet> {
        self.audio.iter()
    }
    #[must_use]
    pub fn audio_mut(&mut self) -> EntityIterMut<AudioSwitchingSet> {
        self.audio.iter_mut()
    }

    #[must_use]
    pub fn metadata(&self) -> EntityIter<MetadataSwitchingSet> {
        self.metadata.iter()
    }

    #[must_use]
    pub fn video(&self) -> EntityIter<VideoSwitchingSet> {
        self.video.iter()
    }

    #[must_use]
    pub fn video_mut(&mut self) -> EntityIterMut<VideoSwitchingSet> {
        self.video.iter_mut()
    }

    #[must_use]
    pub fn video_switching_set(&self, switching_set_id: &str) -> Option<&VideoSwitchingSet> {
        self.video.get(switching_set_id)
    }

    #[must_use]
    pub fn audio_switching_set(&self, switching_set_id: &str) -> Option<&AudioSwitchingSet> {
        self.audio.get(switching_set_id)
    }

    #[must_use]
    pub fn video_switching_set_mut(
        &mut self,
        switching_set_id: &str,
    ) -> Option<&mut VideoSwitchingSet> {
        self.video.get_mut(switching_set_id)
    }

    #[must_use]
    pub fn audio_switching_set_mut(
        &mut self,
        switching_set_id: &str,
    ) -> Option<&mut AudioSwitchingSet> {
        self.audio.get_mut(switching_set_id)
    }

    #[must_use]
    pub const fn time_bounds(&self) -> TimeBounds {
        self.time_bounds
    }

    pub(super) fn validate_active(&self) -> Result<()> {
        if self.time_bounds.start_time().is_none() {
            return Err(Error::MissingStartTime(self.id.clone()));
        }
        Ok(())
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
        let audio_iter = Self::track_mut_iterator(&mut self.audio);
        let video_iter = Self::track_mut_iterator(&mut self.video);
        let metadata_iter = Self::track_mut_iterator(&mut self.metadata);

        audio_iter.chain(video_iter).chain(metadata_iter)
    }

    fn track_mut_iterator<T: SwitchingSet>(
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
        let audio_iter = Self::initializable_track_mut_iterator(&mut self.audio);
        let video_iter = Self::initializable_track_mut_iterator(&mut self.video);

        audio_iter.chain(video_iter)
    }

    fn initializable_track_mut_iterator<'a, T, U>(
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

    #[must_use]
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

    #[must_use]
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
}

impl Entity for Presentation {
    fn id(&self) -> &str {
        &self.id
    }
}
