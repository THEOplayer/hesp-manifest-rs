use serde::{Deserialize, Serialize};

pub use fairplay::*;
pub use scheme::ProtectionScheme;
pub use system::*;

mod fairplay;
mod scheme;
mod system;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SwitchingSetProtection {
    #[serde(rename = "type")]
    scheme: ProtectionScheme,
    systems: SwitchingSetProtectionSystemVec,
}
