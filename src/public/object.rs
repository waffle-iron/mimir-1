//! Object
use odpi::opaque::ODPIObject;

/// ODPI-C Object wrapper.
#[derive(Clone)]
pub struct Object {
    /// The ODPI-C Object pointer.
    pub inner: *mut ODPIObject,
}
