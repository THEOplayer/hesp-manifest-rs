use serde::{Deserialize, self, Serialize};

use crate::*;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PresentationMulticastMetadata {
    fec: FecMetadata,
    transport_session_id: u32,
    address: String,
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FecMetadata {
    encoding_id: u8,
    encoding_symbol_length: u32,
    maximum_source_block_length: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(from="Option<PresentationMulticastMetadata>", into="Option<PresentationMulticastMetadata>")]
#[serde(rename="multicast_metadata", rename_all = "camelCase")]
pub enum PresentationTransmission {
    Unicast,
    Multicast (PresentationMulticastMetadata),
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
