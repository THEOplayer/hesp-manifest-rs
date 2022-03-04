use serde::{Deserialize, Serialize};
use url::Url;

use super::{BaseManifest, ManifestData};
use crate::util::{EntityIter, EntityIterMut};
use crate::{
    Error, InitializableTrack, Manifest, ManifestDeserialize, ManifestSerialize, MediaType,
    NtpTime, Presentation, Result, StreamType, SwitchingSet, Track, TrackMulticastMetadata,
    TrackTransmission, TrackUid, UnicastManifest,
};

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
        Some(match track.media_type() {
            MediaType::Video => presentation
                .video_switching_set(track.switching_set_id())?
                .track(track.track_id())?
                .transmission(),
            MediaType::Audio => presentation
                .audio_switching_set(track.switching_set_id())?
                .track(track.track_id())?
                .transmission(),
            MediaType::Metadata => unimplemented!("no multicast support for metadata yet"),
        })
    }

    pub fn track_transmission_mut(&mut self, track: &TrackUid) -> Option<&mut TrackTransmission> {
        let presentation = self.presentation_mut(track.presentation_id())?;
        Some(match track.media_type() {
            MediaType::Video => {
                &mut presentation
                    .video_switching_set_mut(track.switching_set_id())?
                    .track_mut(track.track_id())?
                    .transmission
            }
            MediaType::Audio => {
                &mut presentation
                    .audio_switching_set_mut(track.switching_set_id())?
                    .track_mut(track.track_id())?
                    .transmission
            }
            MediaType::Metadata => unimplemented!("no multicast support for metadata yet"),
        })
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

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ManifestMulticastMetadata {
    pub route_version: u8,
    pub fec_encoding_id: u8,
    pub address: String,
    pub expiration_time: NtpTime,
}
