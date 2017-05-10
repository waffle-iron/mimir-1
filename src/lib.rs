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
#[allow(missing_docs)]
pub mod error;
pub mod lob;
pub mod message;
pub mod object;
pub mod objecttype;
pub mod pool;
pub mod statement;
pub mod subscription;
pub mod variable;

mod odpi;
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
    static ref CREDS: Vec<String> = {
        use std::fs::File;
        use std::io::{BufRead, BufReader};
        let file = File::open("/home/jozias/projects/rust-lang/oic-rs/.creds/oic-test")
            .expect("bad creds");
        let mut buf_reader = BufReader::new(file);
        let mut creds = String::new();
        let _ = buf_reader.read_line(&mut creds).expect("bad creds");
        creds.split(":").map(|x| x.trim_right().to_string()).collect()
    };
    static ref CTXT: ContextResult = {
        match context::Context::create() {
            Ok(ctxt) => {
                ContextResult::Ok(ctxt)
            },
            Err(e) => ContextResult::Err(e),
        }
    };
}

#[cfg(test)]
fn error_info(e: error::Error) {
    use std::io::{self, Write};
    writeln!(io::stderr(), "{}", e).expect("badness");
    let ctxt = match *CTXT {
        ContextResult::Ok(ref ctxt) => ctxt,
        ContextResult::Err(ref _e) => return assert!(false),
    };

    let ctxt_error = ctxt.get_error();
    writeln!(io::stderr(), "{}", ctxt_error).expect("badness");
    assert!(false);
}
