use hesp_manifest::{Manifest, ManifestMulticastMetadata, MulticastManifest, UnicastManifest};
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
    assert!(error.contains("missing field"), "Wrong error `{}`", error);
    Ok(())
}

#[test]
fn validate_multicast_manifest() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/")?;
    let input = fs::read_to_string("tests/multicast-manifest.json")?;

    let result = MulticastManifest::from_json(location, &input)?;

    let session_ids: Vec<_> = result
        .multicast_tracks()
        .map(|(meta, _)| meta.transport_session_id)
        .collect();

    assert_eq!(
        result.multicast_metadata(),
        &ManifestMulticastMetadata {
            route_version: 1,
            fec_encoding_id: 5,
            address: "239.0.0.1:6666".parse().unwrap(),
            expiration_time: 10.into(),
        }
    );
    assert_eq!(session_ids, vec![1, 2]);
    Ok(())
}
