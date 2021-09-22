use serde::{Deserialize, Serialize};

use crate::Entity;

use super::*;

#[derive(Deserialize, Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationEvent {
    data: String,
    id: String,
    time_bounds: PresentationEventTimeBounds,
    #[serde(default)]
    encoding: PresentationEventEncoding,
}

impl Entity for PresentationEvent {
    type Id = str;
    fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Deserialize, Debug, Serialize, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PresentationEventTimeBounds {
    #[serde(default)]
    start_time_offset: u64,
    #[serde(default)]
    duration: u64,
    #[serde(default = "TimeBounds::default_scale")]
    scale: u64,
}

#[derive(Deserialize, Debug, Serialize, Copy, Clone, Eq, PartialEq, Hash)]
#[serde(rename_all = "lowercase")]
enum PresentationEventEncoding {
    Identity,
    Base64,
    Json,
}

impl Default for PresentationEventEncoding {
    fn default() -> Self {
        Self::Identity
    }
}
