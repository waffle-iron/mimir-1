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

mod context;
mod error;
mod ffi;
mod odpi;

pub use context::Context;
pub use odpi::flags;
