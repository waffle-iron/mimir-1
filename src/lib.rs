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

mod public;

pub mod context;
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

#[cfg(test)]
pub enum ContextResult {
    Ok(Context),
    Err(error::Error),
}

#[cfg(test)]
unsafe impl Sync for ContextResult {}

#[cfg(test)]
pub enum ConnResult {
    Ok(Connection),
    Err(error::Error),
}

#[cfg(test)]
unsafe impl Sync for ConnResult {}

#[cfg(test)]
lazy_static! {
    static ref CTXT: ContextResult = {
        match Context::new() {
            Ok(mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                ContextResult::Ok(ctxt)
            },
            Err(e) => ContextResult::Err(e),
        }
    };

    static ref CONN: ConnResult = {
        let ctxt = match *CTXT {
            ContextResult::Ok(ref ctxt) => ctxt,
            ContextResult::Err(ref _e) => return ConnResult::Err(
                error::ErrorKind::Statement("".to_string()).into()
            ),
        };
        match Connection::connect(ctxt,
                                    Some("jozias"),
                                    Some("chip18jj"),
                                    "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL") {
            Ok(conn) => ConnResult::Ok(conn),
            Err(e) => ConnResult::Err(e),
        }
    };
}
