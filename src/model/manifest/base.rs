use chrono::{DateTime, FixedOffset};
use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Address, AudioTrack, Error, LiveStream, ManifestData, MetadataTrack, Presentation, Result,
    StreamType, VideoTrack,
};

#[derive(Debug, Clone)]
pub(super) struct BaseManifest {
    pub creation_date: DateTime<FixedOffset>,
    pub fallback_poll_rate: u64,
    pub presentations: EntityMap<Presentation>,
    pub stream_type: StreamType,
}

impl BaseManifest {
    pub const fn stream_type(&self) -> &StreamType {
        &self.stream_type
    }

    pub fn active_presentation(&self) -> Option<&Presentation> {
        match &self.stream_type {
            StreamType::Live(live_data) => self.presentation(&live_data.active_presentation),
            StreamType::Vod => None,
        }
    }

    pub fn audio_tracks(&self) -> impl Iterator<Item = &AudioTrack> {
        self.presentations().flat_map(Presentation::audio_tracks)
    }

    pub fn video_tracks(&self) -> impl Iterator<Item = &VideoTrack> {
        self.presentations().flat_map(Presentation::video_tracks)
    }

    pub fn metadata_tracks(&self) -> impl Iterator<Item = &MetadataTrack> {
        self.presentations().flat_map(Presentation::metadata_tracks)
    }

    pub fn new(location: Url, data: ManifestData) -> Result<Self> {
        let address = Address::new(location, data.content_base_url)?;
        let presentations = data
            .presentations
            .into_iter()
            .map(|p| Presentation::new(&address, p))
            .into_entities()?;
        validate_active(&data.stream_type, &presentations)?;
        let manifest = Self {
            creation_date: data.creation_date.into(),
            fallback_poll_rate: data.fallback_poll_rate.into(),
            presentations,
            stream_type: data.stream_type,
        };

        Ok(manifest)
    }
    pub fn presentations(&self) -> EntityIter<Presentation> {
        self.presentations.iter()
    }
    pub fn presentations_mut(&mut self) -> EntityIterMut<Presentation> {
        self.presentations.iter_mut()
    }
    pub fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.get(id)
    }
    pub fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.presentations.get_mut(id)
    }
}

pub(super) fn validate_active(
    stream_type: &StreamType,
    presentations: &EntityMap<Presentation>,
) -> Result<()> {
    if let StreamType::Live(LiveStream {
        active_presentation,
        current_time,
        ..
    }) = stream_type
    {
        let active_presentation = presentations
            .get(active_presentation)
            .ok_or_else(|| Error::InvalidActivePresentationId(active_presentation.clone()))?;
        active_presentation.validate_active()?;
        if *current_time < active_presentation.time_bounds().start_time().unwrap() {
            return Err(Error::ImpossibleCurrentTime(
                active_presentation.id().to_string(),
            ));
        }
    }
    Ok(())
}
