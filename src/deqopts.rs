//! Deque Options
use odpi::opaque::ODPIDeqOptions;

/// ODPI-C Deque Options wrapper.
#[derive(Clone)]
pub struct DeqOptions {
    /// The ODPI-C DeqOptions pointer.
    pub inner: *mut ODPIDeqOptions,
}
