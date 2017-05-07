//! Enque Options
use odpi::opaque::ODPIEnqOptions;

/// ODPI-C Enque Options wrapper.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C EnqOptions pointer.
    pub inner: *mut ODPIEnqOptions,
}
