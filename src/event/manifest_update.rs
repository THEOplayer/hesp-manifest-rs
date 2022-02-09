use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ManifestUpdate {
    pub url: Url,
}

impl ManifestUpdate {
    pub const VALUE: &'static str = "manifestupdate";
}
