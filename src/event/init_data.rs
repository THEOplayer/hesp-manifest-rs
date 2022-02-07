use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct InitData {
    pub index: u64,
    pub offset: u64,
}

impl InitData {
    pub const VALUE: &'static str = "initdata";
}
