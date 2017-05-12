// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Subscription handles are used to represent subscriptions to events such as continuous query
//! notification and object change notification. They are created by calling the function
//! `Connection::new_subscription()` and are destroyed by calling the function
//! `Subscription::close()` or releasing the last reference by calling the function
//! `Subscription::release()`.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPISubscr;

/// ODPI-C Message Props wrapper.
#[derive(Clone)]
pub struct Subscription {
    /// The ODPI-C MsgProps pointer.
    inner: *mut ODPISubscr,
    /// The subscription id.
    id: u32,
}

impl Subscription {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPISubscr {
        self.inner
    }

    ///
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiSubscr_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Subscription("dpiSubscr_addRef".to_string()))
    }

    ///
    pub fn set_id(&mut self, id: u32) -> &mut Subscription {
        self.id = id;
        self
    }
}

impl From<*mut ODPISubscr> for Subscription {
    fn from(inner: *mut ODPISubscr) -> Subscription {
        Subscription {
            inner: inner,
            id: 0,
        }
    }
}
