use hesp_manifest::{Manifest, UnicastManifest};
use std::fs;
use url::Url;

#[test]
fn deserialize_v1_0_0_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/v1_0_0-manifest.json")?;
    UnicastManifest::from_json(location, &input)?;

    Ok(())
}

#[test]
fn deserialize_v1_1_0_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/v1_1_0-manifest.json")?;
    UnicastManifest::from_json(location, &input)?;

    Ok(())
}

#[test]
fn deserialize_v2_0_0_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/v2_0_0-manifest.json")?;
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
    assert!(error.contains("missing field"), "Wrong error `{error}`");
    Ok(())
}
