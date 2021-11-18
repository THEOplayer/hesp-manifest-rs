use serde::{self, Deserialize, Serialize};

use crate::TransferObjectIdentifierLimits;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Copy)]
pub enum TransmissionType {
    Unicast,
    Multicast,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Copy)]
#[serde(
    from = "Option<TransferObjectIdentifierLimits>",
    into = "Option<TransferObjectIdentifierLimits>"
)]
#[serde(rename = "toi_limits", rename_all = "camelCase")]
pub enum TrackTransmission {
    Unicast,
    Multicast {
        toi_limits: TransferObjectIdentifierLimits,
    },
}

impl From<Option<TransferObjectIdentifierLimits>> for TrackTransmission {
    fn from(input: Option<TransferObjectIdentifierLimits>) -> Self {
        match input {
            None => Self::Unicast,
            Some(toi_limits) => Self::Multicast { toi_limits },
        }
    }
}

impl From<TrackTransmission> for Option<TransferObjectIdentifierLimits> {
    fn from(input: TrackTransmission) -> Self {
        match input {
            TrackTransmission::Unicast => None,
            TrackTransmission::Multicast { toi_limits } => Some(toi_limits),
        }
    }
}

impl TrackTransmission {
    pub const fn get_type(&self) -> TransmissionType {
        match self {
            TrackTransmission::Unicast => TransmissionType::Unicast,
            TrackTransmission::Multicast { .. } => TransmissionType::Multicast,
        }
    }
}
