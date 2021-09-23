use crate::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

pub trait Manifest {
    fn presentations(&self) -> &[Presentation];
    fn presentations_mut(&mut self) -> &mut [Presentation];
    fn content_base_url(&self) -> Option<&RelativeBaseUrl>;
    fn content_base_url_mut(&mut self) -> Option<&mut RelativeBaseUrl>;

    fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations().iter().find(|p| p.id() == id)
    }

    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation> {
        self.presentations_mut().iter_mut().find(|p| p.id() == id)
    }
}

validate_on_deserialize!(UnicastManifest);
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct UnicastManifest {
    pub(super) creation_date: DateTime,
    pub(super) fallback_poll_rate: Number,
    pub(super) manifest_version: ManifestVersion,
    pub(super) presentations: EntityVec<Presentation>,
    #[serde(flatten)]
    pub(super) stream_type: UnicastStreamType,
    pub(super) content_base_url: Option<RelativeBaseUrl>,
}

impl UnicastManifest {
    pub fn stream_type(&self) -> &UnicastStreamType {
        &self.stream_type
    }

    pub fn active_presentation(&self) -> Option<&Presentation> {
        match &self.stream_type {
            UnicastStreamType::Live(live_data) => self.presentation(&live_data.active_presentation),
            _ => None,
        }
    }
}

impl Manifest for UnicastManifest {
    fn presentations(&self) -> &[Presentation] {
        &self.presentations
    }

    fn presentations_mut(&mut self) -> &mut [Presentation] {
        &mut self.presentations
    }

    fn content_base_url(&self) -> Option<&RelativeBaseUrl> {
        self.content_base_url.as_ref()
    }

    fn content_base_url_mut(&mut self) -> Option<&mut RelativeBaseUrl> {
        self.content_base_url.as_mut()
    }
}

impl Validate for UnicastManifest {
    fn validate(&self) -> Result<()> {
        if let UnicastStreamType::Live(LiveStream {
            active_presentation,
            ..
        }) = &self.stream_type
        {
            self.presentation(active_presentation)
                .ok_or_else(|| Error::InvalidActivePresentationId(active_presentation.to_owned()))?
                .validate_active()?;
        }
        for presentation in self.presentations() {
            presentation.ensure_unicast()?;
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum UnicastStreamType {
    Live(LiveStream),
    Vod,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LiveStream {
    pub availability_duration: ScaledValue,
    pub active_presentation: String,
    pub time_source: Option<TimeSource>,
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
                "activePresentation": "0",
            }"#;
        let result = serde_json::from_str::<UnicastManifest>(data);

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
        let result = serde_json::from_str::<UnicastManifest>(data);

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
