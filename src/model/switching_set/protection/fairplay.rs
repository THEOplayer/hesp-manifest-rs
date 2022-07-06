use super::ProtectionSystemData;
use itertools::Itertools;
use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use uuid::{uuid, Uuid};

use crate::{Error, Result};

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct Fairplay {
    pub uri: String,
    pub keyformat: String,
    pub keyformatversions: KeyFormatVersions,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct KeyFormatVersions(Vec<u8>);

impl Fairplay {
    pub const SCHEME_ID: Uuid = uuid!("94ce86fb-07ff-4f43-adb8-93d2fa968ca2");
}

impl TryFrom<ProtectionSystemData> for Fairplay {
    type Error = Error;

    fn try_from(mut value: ProtectionSystemData) -> Result<Self> {
        if value.scheme_id == Fairplay::SCHEME_ID {
            Ok(Fairplay {
                uri: value
                    .attributes
                    .remove("uri")
                    .ok_or(Error::MissingFairplayAttribute("uri"))?,
                keyformat: value
                    .attributes
                    .remove("keyformat")
                    .ok_or(Error::MissingFairplayAttribute("keyformat"))?,
                keyformatversions: value
                    .attributes
                    .remove("keyformatversions")
                    .ok_or(Error::MissingFairplayAttribute("keyformatversions"))?
                    .parse()?,
            })
        } else {
            Err(Error::FairplaySchemeId)
        }
    }
}

impl From<Fairplay> for ProtectionSystemData {
    fn from(input: Fairplay) -> Self {
        ProtectionSystemData {
            scheme_id: Fairplay::SCHEME_ID,
            attributes: HashMap::from([
                ("uri".to_string(), input.uri.to_string()),
                ("keyformat".to_string(), input.keyformat.to_string()),
                (
                    "keyformatversions".to_string(),
                    input.keyformatversions.to_string(),
                ),
            ]),
        }
    }
}

impl FromStr for KeyFormatVersions {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        s.split('/')
            .map(|v| v.parse().map_err(Error::KeyFormatVersion))
            .collect::<Result<_>>()
            .map(Self)
    }
}

impl fmt::Display for KeyFormatVersions {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt.write_str(&self.0.iter().join("/"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{ProtectionSystemAttributes, SwitchingSetProtectionSystem};
    use anyhow::Result;

    #[test]
    fn serialize_fairplay() -> Result<()> {
        let src = SwitchingSetProtectionSystem {
            pssh: None,
            attributes: ProtectionSystemAttributes::Fairplay(Fairplay {
                uri: "https://example.com/fairplay.xml".to_string(),
                keyformat: "com.apple.fairplay.v1".to_string(),
                keyformatversions: KeyFormatVersions(vec![1, 2, 5]),
            }),
        };
        let json = serde_json::to_string(&src)?;
        assert!(json.contains("\"schemeId\":\"94ce86fb-07ff-4f43-adb8-93d2fa968ca2\""));
        assert!(json.contains("\"uri\":\"https://example.com/fairplay.xml\""));
        assert!(json.contains("\"keyformat\":\"com.apple.fairplay.v1\""));
        assert!(json.contains("\"keyformatversions\":\"1/2/5\""));
        Ok(())
    }

    #[test]
    fn deserialize_fairplay() -> Result<()> {
        let data = r#"
        {
            "schemeId":"94ce86fb-07ff-4f43-adb8-93d2fa968ca2",
            "uri":"https://example.com/fairplay.xml",
            "keyformat":"com.apple.fairplay.v1",
            "keyformatversions":"1/2/5"
        }"#;
        let system: SwitchingSetProtectionSystem = serde_json::from_str(data)?;

        match system.attributes {
            ProtectionSystemAttributes::Fairplay(fairplay) => {
                assert_eq!(fairplay.uri, "https://example.com/fairplay.xml");
                assert_eq!(fairplay.keyformat, "com.apple.fairplay.v1");
                assert_eq!(fairplay.keyformatversions, KeyFormatVersions(vec![1, 2, 5]));
            }
            ProtectionSystemAttributes::Generic { .. } => {
                panic!("Expected Fairplay")
            }
        }
        Ok(())
    }
}
