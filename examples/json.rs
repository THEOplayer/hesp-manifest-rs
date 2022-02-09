use std::fs;

use url::Url;

use hesp_manifest::{Manifest, UnicastManifest};

fn main() -> anyhow::Result<()> {
    let location = Url::parse("https://www.theoplayer.com/manifest.json")?;
    let input = fs::read_to_string("tests/example-manifest.json")?;
    let manifest = UnicastManifest::from_json(location, &input)?;
    let json = serde_json::to_string(&manifest)?;

    println!("{}", json);
    Ok(())
}
