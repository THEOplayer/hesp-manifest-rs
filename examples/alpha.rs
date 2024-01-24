use hesp_manifest::{Manifest, UnicastManifest};
use std::fs;
use url::Url;

/// will return an Err because the alpha manifest does not follow the spec
fn main() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com")?;
    let input = fs::read_to_string("tests/alpha-manifest.json")?;
    let err = UnicastManifest::from_json(location, &input).err().unwrap();

    println!("{err}");
    Ok(())
}
