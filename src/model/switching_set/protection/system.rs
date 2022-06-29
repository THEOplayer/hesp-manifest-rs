use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Error, Fairplay, Result};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GenericSwitchingSetProtectionSystem {
    pub pssh: Option<String>,
    pub scheme_id: Uuid,
    #[serde(flatten)]
    pub attributes: HashMap<String, String>,
}

#[derive(Serialize, Debug, Clone, Eq, PartialEq)]
#[serde(into = "GenericSwitchingSetProtectionSystem")]
pub enum SwitchingSetProtectionSystem {
    Fairplay(Fairplay),
    Generic(GenericSwitchingSetProtectionSystem),
}

impl From<SwitchingSetProtectionSystem> for GenericSwitchingSetProtectionSystem {
    fn from(input: SwitchingSetProtectionSystem) -> Self {
        match input {
            SwitchingSetProtectionSystem::Fairplay(fairplay) => fairplay.into(),
            SwitchingSetProtectionSystem::Generic(generic) => generic,
        }
    }
}

impl<'de> Deserialize<'de> for SwitchingSetProtectionSystem {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let generic = GenericSwitchingSetProtectionSystem::deserialize(deserializer)?;
        if generic.scheme_id == Fairplay::SCHEME_ID {
            let fairplay = generic.try_into().map_err(D::Error::custom)?;
            Ok(SwitchingSetProtectionSystem::Fairplay(fairplay))
        } else {
            Ok(SwitchingSetProtectionSystem::Generic(generic))
        }
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(try_from = "Vec<SwitchingSetProtectionSystem>")]
pub struct SwitchingSetProtectionSystemVec(Vec<SwitchingSetProtectionSystem>);

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
