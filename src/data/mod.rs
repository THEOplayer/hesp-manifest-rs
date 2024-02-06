use serde::{Deserialize, Serialize};

pub use v2_0_0::*;

use crate::Manifest;

mod v1_0_0;
mod v1_1_0;
mod v2_0_0;

#[derive(Deserialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestDeserialize {
    #[serde(rename = "1.0.0")]
    V1_0_0(v1_0_0::ManifestData),
    #[serde(rename = "1.1.0")]
    V1_1_0(v1_1_0::ManifestData),
    #[serde(rename = "2.0.0")]
    V2_0_0(v2_0_0::ManifestData),
}

#[derive(Serialize, Debug, Clone)]
#[serde(tag = "manifestVersion")]
pub enum ManifestSerialize {
    #[serde(rename = "2.0.0")]
    V2_0_0(v2_0_0::ManifestData),
}

impl From<Manifest> for ManifestSerialize {
    fn from(input: Manifest) -> Self {
        Self::V2_0_0(ManifestData::from(input))
    }
}
