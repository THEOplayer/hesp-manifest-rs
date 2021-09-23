use serde::{self, Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationMulticastMetadata {
    fec: FecMetadata,
    transport_session_id: u32,
    address: String,
}

impl PresentationMulticastMetadata {
    pub fn new(fec: FecMetadata, transport_session_id: u32, address: String) -> Self {
        Self {
            fec,
            transport_session_id,
            address,
        }
    }

    pub fn transport_session_id(&self) -> u32 {
        self.transport_session_id
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FecMetadata {
    pub encoding_id: u8,
    pub encoding_symbol_length: u32,
    pub maximum_source_block_length: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(
    from = "Option<PresentationMulticastMetadata>",
    into = "Option<PresentationMulticastMetadata>"
)]
#[serde(rename = "multicast_metadata", rename_all = "camelCase")]
pub enum PresentationTransmission {
    Unicast,
    Multicast(PresentationMulticastMetadata),
}

impl From<Option<PresentationMulticastMetadata>> for PresentationTransmission {
    fn from(input: Option<PresentationMulticastMetadata>) -> Self {
        match input {
            None => PresentationTransmission::Unicast,
            Some(data) => PresentationTransmission::Multicast(data),
        }
    }
}

impl From<PresentationTransmission> for Option<PresentationMulticastMetadata> {
    fn from(input: PresentationTransmission) -> Self {
        match input {
            PresentationTransmission::Unicast => None,
            PresentationTransmission::Multicast(data) => Some(data),
        }
    }
}

impl PresentationTransmission {
    pub fn get_type(&self) -> TransmissionType {
        match self {
            PresentationTransmission::Unicast => TransmissionType::Unicast,
            PresentationTransmission::Multicast { .. } => TransmissionType::Multicast,
        }
    }
}
