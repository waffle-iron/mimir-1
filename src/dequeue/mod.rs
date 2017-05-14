// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Dequeue option handles are used to represent the options specified when dequeuing messages using
//! advanced queueing. They are created by calling the function `Connection::new_deq_ptions()` and
//! are destroyed by releasing the last reference by calling the function
//! `dequeue::Options::release()`.
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIDeqOptions;

/// Dequeue option handles are used to represent the options specified when dequeuing messages using
/// advanced queueing.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C DeqOptions pointer.
    inner: *mut ODPIDeqOptions,
}

impl Options {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIDeqOptions {
        self.inner
    }

    /// Adds a reference to the dequeue options. This is intended for situations where a reference
    /// to the dequeue options needs to be maintained independently of the reference returned when
    /// the handle was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_addRef(self.inner),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_addRef".to_string()))
    }

    /// Returns the mode that is to be used when dequeuing messages.
    pub fn get_mode(&self) -> Result<flags::ODPIDeqMode> {
        let mut deq_mod_ptr = flags::ODPIDeqMode::Browse;

        try_dpi!(externs::dpiDeqOptions_getMode(self.inner, &mut deq_mod_ptr),
                 Ok(deq_mod_ptr),
                 ErrorKind::DeqOptions("dpiDeqOptions_getMode".to_string()))
    }

    /// Releases a reference to the dequeue options. A count of the references to the dequeue
    /// options is maintained and when this count reaches zero, the memory associated with the
    /// options is freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_release(self.inner),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_release".to_string()))
    }
}

impl From<*mut ODPIDeqOptions> for Options {
    fn from(inner: *mut ODPIDeqOptions) -> Options {
        Options { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags::ODPIConnCloseMode::*;
    use std::ffi::CString;
    use test::CREDS;

    fn dequeue_opts_res() -> Result<()> {
        let ctxt = Context::create()?;

        let mut ccp = ctxt.init_common_create_params()?;
        let enc_cstr = CString::new("UTF-8").expect("badness");
        ccp.set_encoding(enc_cstr.as_ptr());
        ccp.set_nchar_encoding(enc_cstr.as_ptr());

        let conn = Connection::create(&ctxt,
                                      Some(&CREDS[0]),
                                      Some(&CREDS[1]),
                                      Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                      Some(ccp),
                                      None)?;

        conn.add_ref()?;

        let dequeue_opts = conn.new_deq_options()?;
        dequeue_opts.add_ref()?;

        dequeue_opts.release()?;

        conn.release()?;
        conn.close(DefaultClose, None)?;

        Ok(())
    }

    #[test]
    pub fn dequeue_opts() {
        use std::io::{self, Write};

        match dequeue_opts_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
