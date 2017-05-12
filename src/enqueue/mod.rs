// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Enque Options
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIEnqOptions;

/// ODPI-C Enque Options wrapper.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C EnqOptions pointer.
    inner: *mut ODPIEnqOptions,
}

impl Options {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIEnqOptions {
        self.inner
    }

    /// Returns whether the message being enqueued is part of the current transaction or constitutes
    /// a transaction on its own.
    pub fn get_visibility(&self) -> Result<flags::ODPIVisibility> {
        let mut enq_vis_ptr = flags::ODPIVisibility::Immediate;

        try_dpi!(externs::dpiEnqOptions_getVisibility(self.inner, &mut enq_vis_ptr),
                 Ok(enq_vis_ptr),
                 ErrorKind::EnqOptions("dpiEnqOptions_getMode".to_string()))
    }
}

impl From<*mut ODPIEnqOptions> for Options {
    fn from(inner: *mut ODPIEnqOptions) -> Options {
        Options { inner: inner }
    }
}
