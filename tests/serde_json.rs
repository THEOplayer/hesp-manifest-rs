use hesp_manifest::{
    Manifest, MulticastManifest, TrackUid, TransferObjectIdentifierLimits, UnicastManifest,
};
use std::collections::HashMap;
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
fn validate_empty_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/empty-manifest.json")?;

    let result = UnicastManifest::from_json(location, &input);

    assert!(result.is_err());
    let error = result.unwrap_err().to_string();
    assert!(error.contains("missing field"), "Wrong error `{}`", error);
    Ok(())
}

#[test]
fn validate_multicast_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/multicast-manifest.json")?;

    let result = MulticastManifest::from_json(location, &input)?;

    let toi_limits: HashMap<&TrackUid, TransferObjectIdentifierLimits> =
        result.all_toi_limits().collect();
    assert_eq!(toi_limits.len(), 5);
    Ok(())
}
