use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use uuid::Uuid;

use crate::{Error, Fairplay, Result};

#[skip_serializing_none]
#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
pub struct SwitchingSetProtectionSystem {
    pub(super) pssh: Option<String>,
    #[serde(flatten)]
    pub(super) attributes: ProtectionSystemAttributes,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(
    untagged,
    try_from = "ProtectionSystemData",
    into = "ProtectionSystemData"
)]
pub enum ProtectionSystemAttributes {
    Fairplay(Fairplay),
    Generic {
        scheme_id: Uuid,
        attributes: HashMap<String, String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(super) struct ProtectionSystemData {
    pub scheme_id: Uuid,
    #[serde(flatten)]
    pub attributes: HashMap<String, String>,
}

impl SwitchingSetProtectionSystem {
    #[must_use]
    pub fn pssh(&self) -> Option<&str> {
        self.pssh.as_deref()
    }

    #[must_use]
    pub fn scheme_id(&self) -> Uuid {
        match &self.attributes {
            ProtectionSystemAttributes::Fairplay(_) => Fairplay::SCHEME_ID,
            ProtectionSystemAttributes::Generic { scheme_id, .. } => *scheme_id,
        }
    }

    #[must_use]
    pub fn attributes(&self) -> &ProtectionSystemAttributes {
        &self.attributes
    }
}

impl TryFrom<ProtectionSystemData> for ProtectionSystemAttributes {
    type Error = Error;

    fn try_from(value: ProtectionSystemData) -> Result<Self> {
        if value.scheme_id == Fairplay::SCHEME_ID {
            Fairplay::try_from(value).map(ProtectionSystemAttributes::Fairplay)
        } else {
            Ok(ProtectionSystemAttributes::Generic {
                scheme_id: value.scheme_id,
                attributes: value.attributes,
            })
        }
    }
}

impl From<ProtectionSystemAttributes> for ProtectionSystemData {
    fn from(input: ProtectionSystemAttributes) -> Self {
        match input {
            ProtectionSystemAttributes::Fairplay(fairplay) => fairplay.into(),
            ProtectionSystemAttributes::Generic {
                scheme_id,
                attributes,
            } => ProtectionSystemData {
                scheme_id,
                attributes,
            },
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
