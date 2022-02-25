use serde::Serialize;
use url::Url;

use super::BaseManifest;
use crate::util::{EntityIter, EntityIterMut};
use crate::{
    AudioTrack, Error, Manifest, ManifestData, ManifestDeserialize, ManifestSerialize,
    MetadataTrack, Presentation, Result, StreamType, VideoTrack,
};

#[derive(Debug, Clone, Serialize)]
#[serde(into = "ManifestSerialize")]
pub struct UnicastManifest {
    pub(super) inner: BaseManifest,
}

impl UnicastManifest {
    pub fn audio_tracks(&self) -> impl Iterator<Item = &AudioTrack> {
        self.inner.audio_tracks()
    }

    pub fn video_tracks(&self) -> impl Iterator<Item = &VideoTrack> {
        self.inner.video_tracks()
    }

    pub fn metadata_tracks(&self) -> impl Iterator<Item = &MetadataTrack> {
        self.inner.metadata_tracks()
    }

    pub fn active_presentation(&self) -> Option<&Presentation> {
        self.inner.active_presentation()
    }
}
impl Manifest for UnicastManifest {
    fn new(location: Url, data: ManifestData) -> Result<Self> {
        let inner = BaseManifest::new(location, data)?;
        for presentation in &inner.presentations {
            presentation.ensure_unicast()?;
        }
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

    fn from_json(location: Url, json: &str) -> Result<Self> {
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let data = match serde_path_to_error::deserialize(deserializer)? {
            ManifestDeserialize::V1_0_0(data) => data.into(),
            ManifestDeserialize::V1_1_0(data) => data,
            ManifestDeserialize::V1_1_0Multicast(_) => {
                return Err(Error::InvalidUnicastVersion("1.1.0-multicast"))
            }
        };
        Self::new(location, data)
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
