use chrono::{DateTime, FixedOffset};
use std::net::SocketAddr;

use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::{EntityIter, EntityIterMut};
use crate::{
    Error, InitializableTrack, Manifest, ManifestDeserialize, ManifestSerialize, MediaType,
    NtpTime, Presentation, Result, StreamType, SwitchingSet, Track, TrackMulticastMetadata,
    TrackTransmission, TrackUid, UnicastManifest,
};

use super::{BaseManifest, ManifestData};

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestSerialize")]
pub struct MulticastManifest {
    pub(super) inner: BaseManifest,
}

impl MulticastManifest {
    #[must_use]
    pub fn active_presentation(&self) -> &Presentation {
        self.inner.active_presentation().unwrap()
    }

    #[must_use]
    pub fn track_transmission(&self, track: &TrackUid) -> Option<&TrackTransmission> {
        let presentation = self.presentation(track.presentation_id())?;
        match track.media_type() {
            MediaType::Video => Some(
                presentation
                    .video_switching_set(track.switching_set_id())?
                    .track(track.track_id())?
                    .transmission(),
            ),
            MediaType::Audio => Some(
                presentation
                    .audio_switching_set(track.switching_set_id())?
                    .track(track.track_id())?
                    .transmission(),
            ),
            MediaType::Metadata => None,
        }
    }

    pub fn track_transmission_mut(&mut self, track: &TrackUid) -> Option<&mut TrackTransmission> {
        let presentation = self.presentation_mut(track.presentation_id())?;
        match track.media_type() {
            MediaType::Video => Some(
                &mut presentation
                    .video_switching_set_mut(track.switching_set_id())?
                    .track_mut(track.track_id())?
                    .transmission,
            ),
            MediaType::Audio => Some(
                &mut presentation
                    .audio_switching_set_mut(track.switching_set_id())?
                    .track_mut(track.track_id())?
                    .transmission,
            ),
            MediaType::Metadata => None,
        }
    }

    pub fn multicast_tracks(
        &self,
    ) -> impl Iterator<Item = (&TrackMulticastMetadata, &dyn InitializableTrack)> + '_ {
        self.presentations()
            .flat_map(Presentation::multicast_tracks)
    }

    pub fn from_unicast(
        manifest: UnicastManifest,
        multicast_metadata: ManifestMulticastMetadata,
    ) -> Result<Self> {
        let mut inner = manifest.inner;
        if !matches!(inner.stream_type, StreamType::Live(_)) {
            return Err(Error::InvalidMulticastStreamType);
        }
        inner.multicast_metadata = Some(multicast_metadata);
        Ok(Self { inner })
    }

    #[must_use]
    pub fn multicast_metadata(&self) -> &ManifestMulticastMetadata {
        self.inner.multicast_metadata.as_ref().unwrap()
    }

    #[must_use]
    pub fn is_expired(&self) -> bool {
        self.multicast_metadata().expiration_time <= NtpTime::now()
    }

    pub fn initializable_tracks(&mut self) -> impl Iterator<Item = &dyn InitializableTrack> {
        self.presentations()
            .flat_map(Presentation::initializable_tracks)
    }

    pub fn initializable_tracks_mut(
        &mut self,
    ) -> impl Iterator<Item = &mut dyn InitializableTrack> {
        self.presentations_mut()
            .flat_map(Presentation::initializable_tracks_mut)
    }
}

impl Manifest for MulticastManifest {
    fn new(location: Url, data: ManifestData) -> Result<Self> {
        if data.multicast_metadata.is_none() {
            return Err(Error::MissingManifestMulticastMetadata);
        }
        BaseManifest::new(location, data).map(|inner| Self { inner })
    }

    fn presentations(&self) -> EntityIter<Presentation> {
        self.inner.presentations()
    }

    fn presentations_mut(&mut self) -> EntityIterMut<Presentation> {
        self.inner.presentations_mut()
    }

    fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.inner.presentation(id)
    }

    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.inner.presentation_mut(id)
    }

    fn stream_type(&self) -> &StreamType {
        self.inner.stream_type()
    }

    fn creation_date(&self) -> DateTime<FixedOffset> {
        *self.inner.creation_date
    }

    fn fallback_poll_rate(&self) -> u64 {
        self.inner.fallback_poll_rate
    }

    fn from_json(location: Url, json: &str) -> Result<Self> {
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let data = match serde_path_to_error::deserialize(deserializer)? {
            ManifestDeserialize::V1_1_0Multicast(data) => data,
            ManifestDeserialize::V1_0_0(_) => return Err(Error::InvalidMulticastVersion("1.0.0")),
            ManifestDeserialize::V1_1_0(_) => return Err(Error::InvalidMulticastVersion("1.1.0")),
        };
        Self::new(location, data)
    }
}

impl From<MulticastManifest> for UnicastManifest {
    fn from(input: MulticastManifest) -> Self {
        let MulticastManifest { mut inner, .. } = input;
        for presentation in &mut inner.presentations {
            presentation.set_unicast();
        }
        Self { inner }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ManifestMulticastMetadata {
    pub route_version: u8,
    pub fec_encoding_id: u8,
    pub address: SocketAddr,
    pub expiration_time: NtpTime,
}
