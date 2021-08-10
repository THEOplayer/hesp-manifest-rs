use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use crate::*;

validate_on_deserialize!(Manifest);
#[skip_serializing_none]
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase", remote = "Self")]
pub struct Manifest {
    creation_date: DateTime,
    fallback_poll_rate: Number,
    manifest_version: ManifestVersion,
    presentations: EntityVec<Presentation>,
    #[serde(flatten)]
    stream_type: StreamType,
    content_base_url: Option<RelativeBaseUrl>,
}


impl Manifest {
    pub fn stream_type(&self) -> &StreamType { &self.stream_type }
    pub fn presentations(&self) -> &[Presentation] { &self.presentations }
    pub fn content_base_url(&self) -> &Option<RelativeBaseUrl> { &self.content_base_url }
}

impl Validate for Manifest {
    fn validate(&self) -> Result<()> {
        if let StreamType::Live { active_presentation, .. } = &self.stream_type {
            self.presentations
                .iter()
                .find(|p| p.id() == active_presentation)
                .ok_or_else(||Error::InvalidActivePresentationId(active_presentation.to_owned()))?
                .validate_active()
        } else {
            Ok(())
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash)]
enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0Multicast,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum StreamType {
    #[serde(rename_all = "camelCase")]
    Live {
        availability_duration: ScaledValue,
        active_presentation: String,
        time_source: Option<TimeSource>,
    },
    Vod,
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

