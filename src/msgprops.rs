//! Message Properties
use odpi::opaque::ODPIMsgProps;

/// ODPI-C Message Props wrapper.
#[derive(Clone)]
pub struct MsgProps {
    /// The ODPI-C MsgProps pointer.
    pub inner: *mut ODPIMsgProps,
}
