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

#[cfg(test)]
extern crate rand;

// Public API
pub mod common;
pub mod connection;
pub mod context;
pub mod data;
pub mod dequeue;
pub mod enqueue;
pub mod lob;
pub mod message;
pub mod object;
pub mod objecttype;
pub mod statement;
pub mod subscription;
pub mod variable;

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
