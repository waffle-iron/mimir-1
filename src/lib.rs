//! Bindings over Oracle Instant Client for rust
#![deny(missing_docs)]
#![feature(untagged_unions)]
#![recursion_limit="128"]
#![cfg_attr(feature = "cargo-clippy", allow(unseparated_literal_suffix))]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;

mod public;

mod deqopts;
mod enqopts;
mod error;
mod msgprops;
mod object;
mod odpi;
mod util;

pub use error::from_dpi_context;
pub use public::connection::Connection;
pub use public::context::Context;
pub use public::statement::Statement;
pub use public::VersionInfo;
