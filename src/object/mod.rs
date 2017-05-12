// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Object
use odpi::opaque::ODPIObject;

/// ODPI-C Object wrapper.
#[derive(Clone)]
pub struct Object {
    /// The ODPI-C Object pointer.
    pub inner: *mut ODPIObject,
}

impl From<*mut ODPIObject> for Object {
    fn from(inner: *mut ODPIObject) -> Object {
        Object { inner: inner }
    }
}
