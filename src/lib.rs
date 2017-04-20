//! Bindings over Oracle Instant Client for rust
#![deny(missing_docs)]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[macro_use]
mod macros;

extern crate core;
extern crate libc;

pub mod connection;
mod context;
mod error;
mod odpi;
pub mod version;

pub use context::Context;
pub use odpi::flags;
