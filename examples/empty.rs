use hesp_manifest::Manifest;
use std::fs;
use url::Url;

fn main() -> anyhow::Result<()> {
    let location = Url::parse("http://localhost")?;
    let input = fs::read_to_string("tests/empty-manifest.json")?;
    let err = Manifest::from_json(location, &input).err().unwrap();

    println!("{err}");
    Ok(())
}
