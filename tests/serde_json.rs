use hesp_manifest::{Manifest, UnicastManifest};
use serde_json::Value;
use std::fs;
use url::Url;

#[test]
fn deserialize_example_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/example-manifest.json")?;
    UnicastManifest::from_json(location, &input)?;

    Ok(())
}

#[test]
fn validate_empty_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/empty-manifest.json")?;

    let result = UnicastManifest::from_json(location, &input);

    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("missing field"), "Wrong error `{}`", error);
    Ok(())
}
