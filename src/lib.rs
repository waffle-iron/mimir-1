#![feature(untagged_unions)]
#[macro_use]
extern crate error_chain;
#[macro_use]
mod macros;

extern crate core;
extern crate libc;

mod error;
mod ffi;
mod odpi;

pub use odpi::context::Context;
