// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Deque Options
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIDeqOptions;
use std::mem;

/// ODPI-C Deque Options wrapper.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C DeqOptions pointer.
    inner: *mut ODPIDeqOptions,
}

impl Options {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIDeqOptions {
        self.inner
    }

    /// Returns the mode that is to be used when dequeuing messages.
    pub fn get_mode(&self) -> Result<flags::ODPIDeqMode> {
        let mut deq_mod_ptr = unsafe { mem::uninitialized::<flags::ODPIDeqMode>() };

        try_dpi!(externs::dpiDeqOptions_getMode(self.inner, &mut deq_mod_ptr),
                 Ok(deq_mod_ptr),
                 ErrorKind::DeqOptions("dpiDeqOptions_getMode".to_string()))
    }
}

impl From<*mut ODPIDeqOptions> for Options {
    fn from(inner: *mut ODPIDeqOptions) -> Options {
        Options { inner: inner }
    }
}
