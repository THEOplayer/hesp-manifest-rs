use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use uuid::Uuid;

use crate::{Error, Result};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwitchingSetProtection {
    #[serde(rename = "type")]
    scheme: ProtectionScheme,
    systems: SwitchingSetProtectionSystemVec,
}

#[derive(Serialize, Deserialize, Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
enum ProtectionScheme {
    Cenc,
    Cbcs,
}

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
struct SwitchingSetProtectionSystem {
    pssh: Option<String>,
    scheme_id: Uuid,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(try_from = "Vec<SwitchingSetProtectionSystem>")]
struct SwitchingSetProtectionSystemVec(Vec<SwitchingSetProtectionSystem>);

impl TryFrom<Vec<SwitchingSetProtectionSystem>> for SwitchingSetProtectionSystemVec {
    type Error = Error;

    fn try_from(vec: Vec<SwitchingSetProtectionSystem>) -> Result<Self> {
        if vec.is_empty() {
            Err(Error::EmptySwitchingSetProtectionSystems)
        } else {
            Ok(Self(vec))
        }
    }
}
