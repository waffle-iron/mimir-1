//! Bindings over Oracle Instant Client for rust
#![deny(missing_docs)]
#![feature(untagged_unions)]
#![recursion_limit="128"]
#![cfg_attr(feature = "cargo-clippy", allow(unseparated_literal_suffix))]
#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate error_chain;
#[cfg(test)]
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate slog;
#[macro_use]
mod macros;

// Public API
pub mod common;
pub mod connection;
pub mod context;

mod error;
mod odpi;
pub mod pool;
mod util;

#[cfg(test)]
use std::ffi::CString;

#[cfg(test)]
pub enum ContextResult {
    Ok(context::Context),
    Err(error::Error),
}

#[cfg(test)]
unsafe impl Sync for ContextResult {}

#[cfg(test)]
lazy_static! {
    static ref ENC: CString = CString::new("UTF-8").expect("badness");
    static ref CTXT: ContextResult = {
        match context::Context::create() {
            Ok(ctxt) => {
                ContextResult::Ok(ctxt)
            },
            Err(e) => ContextResult::Err(e),
        }
    };
}
