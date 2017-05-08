//! Deque Options
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIEnqOptions;
use std::mem;

/// ODPI-C Deque Options wrapper.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C DeqOptions pointer.
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
        let mut enq_vis_ptr = unsafe { mem::uninitialized::<flags::ODPIVisibility>() };

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
