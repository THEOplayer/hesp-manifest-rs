use serde::{self, Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TrackMulticastMetadata {
    pub fec_encoding_id: u8,
    pub transport_session_id: u32,
    pub address: SocketAddr,
}

impl TrackMulticastMetadata {
    #[must_use]
    pub const fn new(fec_encoding_id: u8, transport_session_id: u32, address: SocketAddr) -> Self {
        Self {
            fec_encoding_id,
            transport_session_id,
            address,
        }
    }
}

#[derive(Clone, Debug)]
pub enum TrackTransmission {
    Unicast,
    Multicast(TrackMulticastMetadata),
}

impl From<Option<TrackMulticastMetadata>> for TrackTransmission {
    fn from(input: Option<TrackMulticastMetadata>) -> Self {
        match input {
            None => Self::Unicast,
            Some(data) => Self::Multicast(data),
        }
    }
}

impl From<TrackTransmission> for Option<TrackMulticastMetadata> {
    fn from(input: TrackTransmission) -> Self {
        match input {
            TrackTransmission::Unicast => None,
            TrackTransmission::Multicast(data) => Some(data),
        }
    }
}
