use crate::*;

//TODO use own error
pub fn deserialize(input: &str) -> std::result::Result<UnicastManifest, serde_path_to_error::Error<serde_json::Error>> {
    let deserializer = &mut serde_json::Deserializer::from_str(input);
    serde_path_to_error::deserialize(deserializer)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use anyhow::Result;

    use super::*;

    #[test]
    fn deserialize_example_manifest() -> Result<()> {
        let input = fs::read_to_string("tests/example-manifest.json")?;

        let result = deserialize(&input)?;
        let json = serde_json::to_string(&result)?;
        deserialize(&json)?;

        Ok(())
    }

    #[test]
    fn validate_empty_manifest() -> Result<()> {
        let input = fs::read_to_string("tests/empty-manifest.json")?;

        let result = deserialize(&input);

        assert!(result.is_err());
        let error = result.unwrap_err().to_string();
        assert!(
            error.contains("missing field"),
            "Wrong error `{}`", error
        );
        Ok(())
    }
}
