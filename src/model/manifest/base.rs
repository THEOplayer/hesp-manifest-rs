use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use url::Url;

use crate::util::{Entity, EntityIter, EntityIterMut, EntityMap, FromEntities};
use crate::{
    Address, AudioTrack, Error, InitializableTrack, LiveStream, ManifestData, ManifestDeserialize,
    ManifestSerialize, MetadataTrack, Presentation, Result, StreamType, Track, TrackUid,
    VideoTrack,
};

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestSerialize")]
pub struct BaseManifest {
    pub creation_date: DateTime<FixedOffset>,
    pub fallback_poll_rate: u64,
    pub presentations: EntityMap<Presentation>,
    pub stream_type: StreamType,
}

impl BaseManifest {
    #[must_use]
    pub const fn stream_type(&self) -> &StreamType {
        &self.stream_type
    }

    #[must_use]
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

    #[must_use]
    pub fn presentations(&self) -> EntityIter<Presentation> {
        self.presentations.iter()
    }

    #[must_use]
    pub fn presentations_mut(&mut self) -> EntityIterMut<Presentation> {
        self.presentations.iter_mut()
    }

    #[must_use]
    pub fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.get(id)
    }
    pub fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.presentations.get_mut(id)
    }

    #[must_use]
    pub fn track(&self, track_uid: &TrackUid) -> Option<&dyn Track> {
        self.presentation(track_uid.presentation_id())?.track(
            track_uid.media_type(),
            track_uid.switching_set_id(),
            track_uid.track_id(),
        )
    }

    #[must_use]
    pub fn track_mut(&mut self, track_uid: &TrackUid) -> Option<&mut dyn Track> {
        self.presentation_mut(track_uid.presentation_id())?
            .track_mut(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    #[must_use]
    pub fn initializable_track(&self, track_uid: &TrackUid) -> Option<&dyn InitializableTrack> {
        self.presentation(track_uid.presentation_id())?
            .initializable_track(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    pub fn initializable_track_mut(
        &mut self,
        track_uid: &TrackUid,
    ) -> Option<&mut dyn InitializableTrack> {
        self.presentation_mut(track_uid.presentation_id())?
            .initializable_track_mut(
                track_uid.media_type(),
                track_uid.switching_set_id(),
                track_uid.track_id(),
            )
    }

    pub fn from_json(location: Url, json: &str) -> Result<Self> {
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let data = match serde_path_to_error::deserialize(deserializer)? {
            ManifestDeserialize::V1_0_0(data) => data.try_into()?,
            ManifestDeserialize::V1_1_0(data) => data.try_into()?,
            ManifestDeserialize::V2_0_0(data) => data,
        };
        Self::new(location, data)
    }
}

fn validate_active(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn duplicate_presentation_id() -> anyhow::Result<()> {
        let data = r#"
            {
                "availabilityDuration": {"value": 1500},
                "creationDate": "2021-03-31T08:00:00.000Z",
                "fallbackPollRate": 300,
                "manifestVersion": "1.0.0",
                "presentations": [
                    {
                        "id": "0",
                        "timeBounds": {"startTime": 0}
                    },
                    {
                        "id": "0",
                        "timeBounds": {"startTime": 0}
                    }
                ],
                "streamType": "vod"
            }"#;
        let location = Url::parse("https://www.theoplayer.com")?;
        let result = BaseManifest::from_json(location, data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("Ids must be unique"),
            "Error did not indicate duplicate presentation id `{error}`",
        );
        Ok(())
    }

    #[test]
    fn validate_active_presentation() -> anyhow::Result<()> {
        let data = r#"
            {
                "availabilityDuration": {"value": 1500},
                "creationDate": "2021-03-31T08:00:00.000Z",
                "fallbackPollRate": 300,
                "manifestVersion": "1.0.0",
                "presentations": [
                    {
                        "id": "0",
                        "timeBounds": {"startTime": 0}
                    }
                ],
                "streamType": "live",
                "activePresentation": "0"
            }"#;
        let location = Url::parse("https://www.theoplayer.com")?;
        let result = BaseManifest::from_json(location, data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("has no currentTime"),
            "Error did not indicate invalid active presentation `{error}`"
        );
        Ok(())
    }
}
