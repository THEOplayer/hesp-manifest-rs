use serde::{Deserialize, Serialize};
use url::Url;

use crate::util::{EntityIter, EntityIterMut, EntityMap};
use crate::*;

use super::data::ManifestData;

// TODO
// impl Validate for UnicastManifest {
//     fn validate(&self) -> Result<()> {
//         if let UnicastStreamType::Live(LiveStream {
//                                            active_presentation,
//                                            ..
//                                        }) = &self.stream_type
//         {
//             self.presentation(active_presentation)
//                 .ok_or_else(|| Error::InvalidActivePresentationId(active_presentation.to_owned()))?
//                 .validate_active()?;
//         }
//         for presentation in self.presentations() {
//             presentation.ensure_unicast()?;
//         }
//         Ok(())
//     }
// }

#[derive(Debug, Clone)]
pub struct UnicastManifest {
    creation_date: DateTime,
    fallback_poll_rate: Number,
    presentations: EntityMap<Presentation>,
    stream_type: UnicastStreamType,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(tag = "streamType", rename_all = "lowercase")]
pub enum UnicastStreamType {
    Live(LiveStream),
    Vod,
}

impl UnicastManifest {
    pub fn new(base_url: &Url, data: ManifestData) -> Result<Self> {
        let url = data.content_base_url.resolve(base_url);
        //TODO check manifest version unicast
        let mut manifest = UnicastManifest {
            creation_date: data.creation_date,
            fallback_poll_rate: data.fallback_poll_rate,
            presentations: data
                .presentations
                .into_iter()
                .map(|p| Presentation::new(url, p))
                .collect(),
            stream_type: data.stream_type,
        };

        Ok(manifest)
    }

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
