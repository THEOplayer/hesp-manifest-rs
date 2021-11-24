use url::Url;

pub use data::ManifestData;
pub use multicast::*;
pub use stream::*;
pub use unicast::*;
pub use version::ManifestVersion;

use crate::util::{EntityIter, EntityIterMut};
use crate::{Presentation, Result};

mod data;
mod multicast;
mod stream;
mod unicast;
mod version;

pub trait Manifest {
    fn new(base_url: &Url, data: ManifestData) -> Result<Self>
    where
        Self: Sized;
    fn presentations(&self) -> EntityIter<Presentation>;
    fn presentations_mut(&mut self) -> EntityIterMut<Presentation>;
    fn presentation(&self, id: &str) -> Option<&Presentation>;
    fn presentation_mut(&mut self, id: &str) -> Option<&mut Presentation>;
    fn stream_type(&self) -> &StreamType;

    fn from_json(base_url: &Url, json: &str) -> Result<Self>
    where
        Self: Sized,
    {
        let deserializer = &mut serde_json::Deserializer::from_str(json);
        let data: ManifestData = serde_path_to_error::deserialize(deserializer)?;
        Self::new(base_url, data)
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::*;

    #[test]
    fn deserialize_example_manifest() -> anyhow::Result<()> {
        let url = Url::parse("https://www.theoplayer.com/")?;
        let input = fs::read_to_string("tests/example-manifest.json")?;

        let result1 = UnicastManifest::from_json(&url, &input)?;
        let output = serde_json::to_string(&result1)?;
        let _result2 = UnicastManifest::from_json(&url, &output)?;

        // assert_eq!(format!("{:?}", result1), format!("{:?}", result2));

        Ok(())
    }

    #[test]
    fn validate_empty_manifest() -> anyhow::Result<()> {
        let url = Url::parse("https://www.theoplayer.com/")?;
        let input = fs::read_to_string("tests/empty-manifest.json")?;

        let result = UnicastManifest::from_json(&url, &input);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(error.contains("missing field"), "Wrong error `{}`", error);
        Ok(())
    }
}
