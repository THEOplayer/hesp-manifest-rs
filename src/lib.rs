pub use error::{Error, Result};
pub use model::*;
pub use parse::deserialize;

mod error;
mod model;
mod parse;
pub mod util;
