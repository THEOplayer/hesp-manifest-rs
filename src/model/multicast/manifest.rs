use std::convert::TryInto;

use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use url::Url;

use crate::*;

validate_on_deserialize!(MulticastManifest);
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct MulticastManifest {
    creation_date: DateTime,
    fallback_poll_rate: Number,
    manifest_version: MulticastManifestVersion,
    presentations: EntityVec<Presentation>,
    #[serde(flatten)]
    stream_type: MulticastStreamType,
    content_base_url: Option<RelativeBaseUrl>,
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
enum MulticastManifestVersion {
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum MulticastStreamType {
    Live(LiveStream),
}

impl MulticastStreamType {
    fn live_data(&self) -> &LiveStream {
        match &self {
            MulticastStreamType::Live(live_data) => live_data,
        }
    }
}

impl StreamType for MulticastStreamType {
    fn is_live(&self) -> bool {
        true
    }

    fn is_vod(&self) -> bool {
        false
    }

    fn active_presentation_id(&self) -> Option<&str> {
        Some(&self.live_data().active_presentation)
    }
}

impl From<MulticastStreamType> for UnicastStreamType {
    fn from(stream_type: MulticastStreamType) -> Self {
        match stream_type {
            MulticastStreamType::Live(data) => UnicastStreamType::Live(data),
        }
    }
}

impl MulticastManifest {
    pub fn presentations(&self) -> &[Presentation] {
        &self.presentations
    }

    pub fn presentations_mut(&mut self) -> &mut [Presentation] {
        &mut self.presentations
    }

    pub fn content_base_url(&self) -> &Option<RelativeBaseUrl> {
        &self.content_base_url
    }

    pub fn content_base_url_mut(&mut self) -> &mut Option<RelativeBaseUrl> {
        &mut self.content_base_url
    }

    pub fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.iter().find(|p| p.id() == id)
    }

    pub fn transport_session_id(&self, presentation_id: &str) -> Option<u32> {
        multicast_tsi(self.presentation(presentation_id)?)
    }

    pub fn all_toi_limits(
        &self,
    ) -> impl Iterator<Item = (TrackPath, TransferObjectIdentifierLimits)> + '_ {
        self.presentations.iter().flat_map(|presentation| {
            let video_toi = presentation.video_tracks().filter_map(multicast_toi);
            let audio_toi = presentation.audio_tracks().filter_map(multicast_toi);
            video_toi.chain(audio_toi)
        })
    }

    pub fn toi_limits(&self, path: &TrackPath) -> Option<TransferObjectIdentifierLimits> {
        self.track_transmission(path)
            .and_then(|transmission| match transmission {
                TrackTransmission::Unicast => None,
                TrackTransmission::Multicast { toi_limits } => Some(toi_limits),
            })
    }

    pub fn track_transmission(&self, path: &TrackPath) -> Option<TrackTransmission> {
        let presentation = self.presentation(path.presentation_id())?;
        Some(*match path.media_type() {
            MediaType::Video => presentation
                .video_switching_set(path.switching_set_id())?
                .track(path.track_id())?
                .transmission(),
            MediaType::Audio => presentation
                .audio_switching_set(path.switching_set_id())?
                .track(path.track_id())?
                .transmission(),
        })
    }

    pub fn transport_session_ids(&self) -> impl Iterator<Item = u32> + '_ {
        self.presentations.iter().filter_map(multicast_tsi)
    }

    pub fn track_info(&self, base_url: &Url) -> impl Iterator<Item = MulticastTrackInfo> + '_ {
        let manifest_url = self.content_base_url.resolve(base_url);
        self.presentations
            .iter()
            .filter_map(|presentation| match presentation.transmission() {
                PresentationTransmission::Unicast => None,
                PresentationTransmission::Multicast(data) => {
                    Some((presentation, data.transport_session_id()))
                }
            })
            .flat_map(move |(presentation, tsi)| {
                let presentation_id = presentation.id();
                let presentation_url = presentation.base_url().resolve(&manifest_url);
                let audio_info = track_info_for_selection_set(
                    tsi,
                    presentation.audio(),
                    presentation_id,
                    presentation_url.clone(),
                );
                let video_info = track_info_for_selection_set(
                    tsi,
                    presentation.video(),
                    presentation_id,
                    presentation_url,
                );
                audio_info.chain(video_info)
            })
    }

    pub fn from_unicast<F>(manifest: UnicastManifest, presentation_transformer: F) -> Result<Self>
    where
        F: FnMut(Presentation) -> Presentation,
    {
        let UnicastManifest {
            creation_date,
            fallback_poll_rate,
            manifest_version: _manifest_version,
            presentations,
            stream_type,
            content_base_url,
        } = manifest;
        let live_data = if let UnicastStreamType::Live(live_data) = stream_type {
            live_data
        } else {
            return Err(Error::InvalidMulticastStreamType);
        };
        let presentations = presentations
            .into_iter()
            .map(presentation_transformer)
            .collect::<Vec<Presentation>>()
            .try_into()?;
        Ok(MulticastManifest {
            creation_date,
            fallback_poll_rate,
            manifest_version: MulticastManifestVersion::V1_0_0,
            presentations,
            stream_type: MulticastStreamType::Live(live_data),
            content_base_url,
        })
    }
}

impl Validate for MulticastManifest {
    fn validate(&self) -> Result<()> {
        let active_id = &self.stream_type.live_data().active_presentation;
        self.presentation(active_id)
            .ok_or_else(|| Error::InvalidActivePresentationId(active_id.to_owned()))?
            .validate_active()
    }
}

impl From<MulticastManifest> for UnicastManifest {
    fn from(input: MulticastManifest) -> Self {
        let MulticastManifest {
            creation_date,
            fallback_poll_rate,
            mut presentations,
            stream_type,
            content_base_url,
            ..
        } = input;
        for presentation in &mut presentations[..] {
            presentation.set_unicast();
        }
        UnicastManifest {
            creation_date,
            fallback_poll_rate,
            manifest_version: ManifestVersion::V1_0_0,
            presentations,
            stream_type: stream_type.into(),
            content_base_url,
        }
    }
}

fn multicast_tsi(presentation: &Presentation) -> Option<u32> {
    match presentation.transmission() {
        PresentationTransmission::Unicast => None,
        PresentationTransmission::Multicast(data) => Some(data.transport_session_id()),
    }
}

fn multicast_toi<T: MediaTrack>(
    (path, track): (TrackPath, &T),
) -> Option<(TrackPath, TransferObjectIdentifierLimits)> {
    match track.transmission() {
        TrackTransmission::Unicast => None,
        &TrackTransmission::Multicast { toi_limits } => Some((path, toi_limits)),
    }
}

fn track_info_for_selection_set<'a, S: MediaSwitchingSet>(
    tsi: u32,
    selection_set: &'a [S],
    presentation_id: &'a str,
    presentation_url: Url,
) -> impl Iterator<Item = MulticastTrackInfo> + 'a {
    selection_set.iter().flat_map(move |switching_set| {
        let switching_set_url = switching_set.base_url().resolve(&presentation_url);
        let switching_set_id = switching_set.id();
        switching_set.tracks().iter().map(move |track| {
            let path = TrackPath::new(
                presentation_id.to_owned(),
                S::MEDIA_TYPE,
                switching_set_id.to_owned(),
                track.id().to_owned(),
            );
            MulticastTrackInfo {
                path,
                base_url: track.base_url().resolve(&switching_set_url),
                initialization_pattern: track.initialization_pattern().clone(),
                continuation_pattern: track.continuation_pattern().clone(),
                tsi,
                toi_limits: unwrap_toi_limits(track),
            }
        })
    })
}

fn unwrap_toi_limits<T: MediaTrack>(track: &T) -> TransferObjectIdentifierLimits {
    match track.transmission() {
        TrackTransmission::Unicast => panic!("multicast presentation cannot contain unicast track"),
        TrackTransmission::Multicast { toi_limits } => *toi_limits,
    }
}
