//! Bindings over Oracle Instant Client for rust
#![deny(missing_docs)]
#![feature(untagged_unions)]
#![cfg_attr(feature = "cargo-clippy", allow(unseparated_literal_suffix))]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[macro_use]
mod macros;

pub mod connection;
mod context;
mod error;
// mod ffi;
mod odpi;
mod util;
pub mod version;

pub use error::from_dpi_context;
pub use context::Context;
