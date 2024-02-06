extern crate core;

pub use error::{Error, Result};
pub use model::*;

pub mod data;
mod error;
pub mod event;
mod model;
pub mod util;
