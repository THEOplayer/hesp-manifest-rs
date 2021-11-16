use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Eq, PartialEq, Hash, Copy, Clone)]
pub enum ManifestVersion {
    #[serde(rename = "1.0.0")]
    V1_0_0,
    #[serde(rename = "1.0.0-multicast")]
    V1_0_0Multicast,
}
