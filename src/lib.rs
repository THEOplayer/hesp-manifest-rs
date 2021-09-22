#[macro_use]
extern crate lazy_static;

pub use error::{Error, Result};
pub use model::*;
pub use parse::deserialize;
pub use util::*;

mod error;
mod model;
mod parse;
mod util;
