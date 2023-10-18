#![feature(associated_type_defaults)]
#![crate_name = "wamp_core"]
#![feature(slice_pattern)]
#![feature(lazy_cell)]
pub mod messages;
pub mod roles;
pub mod error;
pub mod factories;
pub mod uri;

pub use regex;
pub use serde;
pub use serde_json;
pub use serde_repr;
pub use lazy_static;

pub use messages::*;
pub use error::*;
pub use factories::*;
pub use uri::*;