use serde::Serialize;
use url::Url;

use crate::util::{EntityIter, EntityIterMut, EntityMap, FromEntities, RelativeUrl};
use crate::{
    DateTime, Error, LiveStream, Manifest, ManifestVersion, MediaTrack, Number, Presentation,
    PresentationTransmission, Result, SwitchingSet, TrackTransmission, TrackType, TrackUid,
    TransferObjectIdentifierLimits, UnicastManifest, UnicastStreamType,
};

use super::unicast::validate_active;
use super::ManifestData;

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestData")]
pub struct MulticastManifest {
    pub(super) creation_date: DateTime,
    pub(super) fallback_poll_rate: Number,
    pub(super) presentations: EntityMap<Presentation>,
    pub(super) stream_type: MulticastStreamType,
}

#[derive(Debug, Clone)]
pub enum MulticastStreamType {
    Live(LiveStream),
}

impl MulticastStreamType {
    const fn live_data(&self) -> &LiveStream {
        match &self {
            MulticastStreamType::Live(live_data) => live_data,
        }
    }
}

impl From<MulticastStreamType> for UnicastStreamType {
    fn from(stream_type: MulticastStreamType) -> Self {
        match stream_type {
            MulticastStreamType::Live(data) => Self::Live(data),
        }
    }
}

impl TryFrom<UnicastStreamType> for MulticastStreamType {
    type Error = Error;

    fn try_from(value: UnicastStreamType) -> Result<Self> {
        match value {
            UnicastStreamType::Live(stream) => Ok(Self::Live(stream)),
            UnicastStreamType::Vod => Err(Error::InvalidMulticastStreamType),
        }
    }
}

impl MulticastManifest {
    pub fn active_presentation(&self) -> &Presentation {
        self.presentation(&self.stream_type.live_data().active_presentation)
            .unwrap()
    }

    pub const fn stream_type(&self) -> &MulticastStreamType {
        &self.stream_type
    }

    pub fn transport_session_id(&self, presentation_id: &str) -> Option<u32> {
        multicast_tsi(self.presentation(presentation_id)?)
    }

    pub fn all_toi_limits(
        &self,
    ) -> impl Iterator<Item = (&TrackUid, TransferObjectIdentifierLimits)> + '_ {
        fn toi<T: MediaTrack>(track: &T) -> Option<(&TrackUid, TransferObjectIdentifierLimits)> {
            match track.transmission() {
                TrackTransmission::Unicast => None,
                &TrackTransmission::Multicast { toi_limits } => Some((track.uid(), toi_limits)),
            }
        }
        self.presentations.iter().flat_map(|presentation| {
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
            TrackType::Video => presentation
                .video_switching_set(track.switching_set_id())?
                .track(track.track_id())?
                .transmission(),
            TrackType::Audio => presentation
                .audio_switching_set(track.switching_set_id())?
                .track(track.track_id())?
                .transmission(),
            TrackType::Metadata => unimplemented!("no multicast support for metadata yet"),
        })
    }

    pub fn transport_session_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.presentations.iter().filter_map(multicast_tsi)
    }

    pub fn from_unicast<F>(manifest: UnicastManifest, presentation_transformer: F) -> Result<Self>
    where
        F: FnMut(Presentation) -> Presentation,
    {
        let UnicastManifest {
            creation_date,
            fallback_poll_rate,
            presentations,
            stream_type,
        } = manifest;
        let live_data = if let UnicastStreamType::Live(live_data) = stream_type {
            live_data
        } else {
            return Err(Error::InvalidMulticastStreamType);
        };
        let presentations = presentations
            .into_iter()
            .map(presentation_transformer)
            .map(Ok)
            .into_entities()?;
        Ok(Self {
            creation_date,
            fallback_poll_rate,
            presentations,
            stream_type: MulticastStreamType::Live(live_data),
        })
    }
}

impl Manifest for MulticastManifest {
    fn new(base_url: &Url, data: ManifestData) -> Result<Self> {
        let url = data.content_base_url.resolve(base_url)?;
        if data.manifest_version != ManifestVersion::V1_0_0Multicast {
            return Err(Error::InvalidMulticastVersion(data.manifest_version));
        }
        let presentations = data
            .presentations
            .into_iter()
            .map(|p| Presentation::new(&url, p))
            .into_entities()?;
        validate_active(&data.stream_type, &presentations)?;
        let manifest = Self {
            creation_date: data.creation_date,
            fallback_poll_rate: data.fallback_poll_rate,
            presentations,
            stream_type: data.stream_type.try_into()?,
        };
        Ok(manifest)
    }
    fn presentations(&self) -> EntityIter<Presentation> {
        self.presentations.iter()
    }
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation> {
        self.presentations.iter_mut()
    }
    fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.get(id)
    }
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.presentations.get_mut(id)
    }
}

impl From<MulticastManifest> for UnicastManifest {
    fn from(input: MulticastManifest) -> Self {
        let MulticastManifest {
            creation_date,
            fallback_poll_rate,
            mut presentations,
            stream_type,
            ..
        } = input;
        for presentation in presentations.iter_mut() {
            presentation.set_unicast();
        }
        Self {
            creation_date,
            fallback_poll_rate,
            presentations,
            stream_type: stream_type.into(),
        }
    }
}

const fn multicast_tsi(presentation: &Presentation) -> Option<u32> {
    match presentation.transmission() {
        PresentationTransmission::Unicast => None,
        PresentationTransmission::Multicast(data) => Some(data.transport_session_id()),
    }
}
