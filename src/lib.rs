#[macro_use]
extern crate lazy_static;

pub use model::*;
pub use parse::deserialize;
pub use util::*;
pub use error::{Error, Result};

mod model;
mod parse;
mod util;
mod error;
