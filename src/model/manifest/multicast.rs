use serde::Serialize;
use url::Url;

use crate::util::{EntityIter, EntityIterMut, FromEntities};
use crate::{
    Error, Manifest, ManifestVersion, MediaType, Presentation, PresentationTransmission, Result,
    StreamType, SwitchingSet, Track, TrackTransmission, TrackUid, TransferObjectIdentifierLimits,
    UnicastManifest,
};

use super::ManifestData;

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestData")]
pub struct MulticastManifest {
    pub(super) inner: UnicastManifest,
}

impl MulticastManifest {
    pub fn active_presentation(&self) -> &Presentation {
        self.inner.active_presentation().unwrap()
    }

    pub fn transport_session_id(&self, presentation_id: &str) -> Option<u32> {
        multicast_tsi(self.presentation(presentation_id)?)
    }

    pub fn all_toi_limits(
        &self,
    ) -> impl Iterator<Item = (&TrackUid, TransferObjectIdentifierLimits)> + '_ {
        fn toi<T: Track>(track: &T) -> Option<(&TrackUid, TransferObjectIdentifierLimits)> {
            match track.transmission() {
                TrackTransmission::Unicast => None,
                &TrackTransmission::Multicast { toi_limits } => Some((track.uid(), toi_limits)),
            }
        }
        self.presentations().flat_map(|presentation| {
            let video_toi = presentation.video_tracks().filter_map(toi);
            let audio_toi = presentation.audio_tracks().filter_map(toi);
            video_toi.chain(audio_toi)
        })
    }

    pub fn toi_limits(&self, track: &TrackUid) -> Option<TransferObjectIdentifierLimits> {
        self.track_transmission(track)
            .and_then(|transmission| match transmission {
                TrackTransmission::Unicast => None,
                TrackTransmission::Multicast { toi_limits } => Some(toi_limits),
            })
    }

    pub fn track_transmission(&self, track: &TrackUid) -> Option<TrackTransmission> {
        let presentation = self.presentation(track.presentation_id())?;
        Some(*match track.track_type() {
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

    pub fn transport_session_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.presentations().filter_map(multicast_tsi)
    }

    pub fn from_unicast<F>(
        mut manifest: UnicastManifest,
        presentation_transformer: F,
    ) -> Result<Self>
    where
        F: FnMut(Presentation) -> Presentation,
    {
        if !matches!(manifest.stream_type, StreamType::Live(_)) {
            return Err(Error::InvalidMulticastStreamType);
        }
        manifest.presentations = manifest
            .presentations
            .into_iter()
            .map(presentation_transformer)
            .map(Ok)
            .into_entities()?;
        Ok(Self { inner: manifest })
    }
}

impl Manifest for MulticastManifest {
    fn new(base_url: &Url, data: ManifestData) -> Result<Self> {
        if data.manifest_version != ManifestVersion::V1_0_0Multicast {
            return Err(Error::InvalidMulticastVersion(data.manifest_version));
        }
        let inner = UnicastManifest::new(base_url, data)?;
        Ok(Self { inner })
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
}

impl From<MulticastManifest> for UnicastManifest {
    fn from(input: MulticastManifest) -> Self {
        let MulticastManifest { mut inner } = input;
        for presentation in &mut inner.presentations {
            presentation.set_unicast();
        }
        inner
    }
}

const fn multicast_tsi(presentation: &Presentation) -> Option<u32> {
    match presentation.transmission() {
        PresentationTransmission::Unicast => None,
        PresentationTransmission::Multicast(data) => Some(data.transport_session_id()),
    }
}
