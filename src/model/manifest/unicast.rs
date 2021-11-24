use serde::Serialize;
use url::Url;

use crate::util::{EntityIter, EntityIterMut, EntityMap, FromEntities, RelativeUrl};
use crate::{
    AudioTrack, DateTime, Error, LiveStream, Manifest, ManifestData, ManifestVersion,
    MetadataTrack, Number, Presentation, Result, StreamType, VideoTrack,
};

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestData")]
pub struct UnicastManifest {
    pub(super) creation_date: DateTime,
    pub(super) fallback_poll_rate: Number,
    pub(super) presentations: EntityMap<Presentation>,
    pub(super) stream_type: StreamType,
    pub(super) location: Url,
}

impl UnicastManifest {
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
}

impl Manifest for UnicastManifest {
    fn new(location: Url, data: ManifestData) -> Result<Self> {
        if data.manifest_version != ManifestVersion::V1_0_0 {
            return Err(Error::InvalidUnicastVersion(data.manifest_version));
        }
        let url = data.content_base_url.resolve(&location)?;
        let presentations = data
            .presentations
            .into_iter()
            .map(|p| Presentation::new(&url, p))
            .into_entities()?;
        for presentation in &presentations {
            presentation.ensure_unicast()?;
        }
        validate_active(&data.stream_type, &presentations)?;
        let manifest = Self {
            creation_date: data.creation_date,
            fallback_poll_rate: data.fallback_poll_rate,
            presentations,
            stream_type: data.stream_type,
            location,
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

    fn stream_type(&self) -> &StreamType {
        &self.stream_type
    }
}

pub(super) fn validate_active(
    stream_type: &StreamType,
    presentations: &EntityMap<Presentation>,
) -> Result<()> {
    if let StreamType::Live(LiveStream {
        active_presentation,
        ..
    }) = stream_type
    {
        presentations
            .get(active_presentation)
            .ok_or_else(|| Error::InvalidActivePresentationId(active_presentation.clone()))?
            .validate_active()
    } else {
        Ok(())
    }
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
                "streamType": "live",
                "activePresentation": "0"
            }"#;
        let location = Url::parse("https://www.theoplayer.com")?;
        let result = UnicastManifest::from_json(location, data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("Ids must be unique"),
            "Error did not indicate duplicate presentation id `{}`",
            error
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
        let result = UnicastManifest::from_json(location, data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("has no currentTime"),
            "Error did not indicate invalid active presentation `{}`",
            error
        );
        Ok(())
    }
}
