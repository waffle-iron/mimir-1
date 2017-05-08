//! Message Properties
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIMsgProps;
use std::mem;

/// ODPI-C Message Props wrapper.
#[derive(Clone)]
pub struct Properties {
    /// The ODPI-C MsgProps pointer.
    inner: *mut ODPIMsgProps,
}

impl Properties {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIMsgProps {
        self.inner
    }

    /// Returns the mode that was used to deliver the message.
    pub fn get_delivery_mode(&self) -> Result<flags::ODPIMessageDeliveryMode> {
        let mut del_mode_ptr = unsafe { mem::uninitialized::<flags::ODPIMessageDeliveryMode>() };

        try_dpi!(externs::dpiMsgProps_getDeliveryMode(self.inner, &mut del_mode_ptr),
                 Ok(del_mode_ptr.into()),
                 ErrorKind::MsgProps("dpiEnqOptions_getMode".to_string()))
    }
}

impl From<*mut ODPIMsgProps> for Properties {
    fn from(inner: *mut ODPIMsgProps) -> Properties {
        Properties { inner: inner }
    }
}
