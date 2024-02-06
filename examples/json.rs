use std::fs;

use url::Url;

use hesp_manifest::Manifest;

fn main() -> anyhow::Result<()> {
    let location = Url::parse("http://localhost")?;
    let input = fs::read_to_string("tests/v2_0_0-manifest.json")?;
    let manifest = Manifest::from_json(location, &input)?;
    let json = serde_json::to_string_pretty(&manifest)?;

    println!("{json}");
    Ok(())
}
