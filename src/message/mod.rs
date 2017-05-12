// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Message Properties
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIMsgProps;

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
        let mut del_mode_ptr = flags::ODPIMessageDeliveryMode::NotSet;

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
