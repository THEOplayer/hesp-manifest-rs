use url::Url;

use data::ManifestData;
pub use live::LiveStream;
pub use multicast::*;
pub use unicast::*;
pub use version::ManifestVersion;

use crate::util::{EntityIter, EntityIterMut};
use crate::{Presentation, Result};

pub mod data;
mod live;
mod multicast;
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

        let result = UnicastManifest::from_json(&url, &input)?;
        //TODO serialize
        // let json = serde_json::to_string(&result)?;
        // UnicastManifest::deserialize(&url, &json)?;

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
