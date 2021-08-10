use std::fs;

/// will return an Err because the alpha manifest does not follow the spec
fn main() -> anyhow::Result<()> {
    let input = fs::read_to_string("tests/alpha-manifest.json")?;
    let err = hesp_manifest::deserialize(&input).err().unwrap();
    println!("{}", err);
    Ok(())
}