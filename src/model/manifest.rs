use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::*;

validate_on_deserialize!(Manifest);
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct Manifest {
    pub(super) creation_date: DateTime,
    pub(super) fallback_poll_rate: Number,
    pub(super) manifest_version: ManifestVersion,
    pub(super) presentations: EntityVec<Presentation>,
    #[serde(flatten)]
    pub(super) stream_type: StreamType,
    pub(super) content_base_url: Option<RelativeBaseUrl>,
}

impl Manifest {
    pub fn stream_type(&self) -> &StreamType { &self.stream_type }
    pub fn presentations(&self) -> &[Presentation] { &self.presentations }
    pub fn content_base_url(&self) -> &Option<RelativeBaseUrl> { &self.content_base_url }
    pub fn presentation(&self, id: &str) -> Option<&Presentation> {
        self.presentations.iter().find(|p| p.id() == id)
    }
}

impl Validate for Manifest {
    fn validate(&self) -> Result<()> {
        if let StreamType::Live(LiveStream { active_presentation, .. }) = &self.stream_type {
            self.presentation(active_presentation)
                .ok_or_else(|| Error::InvalidActivePresentationId(active_presentation.to_owned()))?
                .validate_active()?;
        }
        for presentation in self.presentations() { presentation.ensure_unicast()?; }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
pub enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum StreamType {
    Live(LiveStream),
    Vod,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LiveStream {
    availability_duration: ScaledValue,
    pub(crate) active_presentation: String,
    time_source: Option<TimeSource>,
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
        let result = serde_json::from_str::<Manifest>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("Ids must be unique"),
            "Error did not indicate duplicate presentation id `{}`", error
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
        let result = serde_json::from_str::<Manifest>(data);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("has no currentTime"),
            "Error did not indicate invalid active presentation `{}`", error
        );
        Ok(())
    }
}

