use std::fs;
use url::Url;
use hesp_manifest::{Manifest, UnicastManifest};

/// will return an Err because the alpha manifest does not follow the spec
fn main() -> anyhow::Result<()> {
    let url = Url::parse("https://www.theoplayer.com")?;
    let input = fs::read_to_string("tests/alpha-manifest.json")?;
    let err = UnicastManifest::from_json(&url, &input).err().unwrap();

    println!("{}", err);
    Ok(())
}
