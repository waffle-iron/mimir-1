// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! This structure represents instances of the types created by the SQL command CREATE OR REPLACE
//! TYPE and is available by handle to a calling application or driver. An object is created by
//! calling the function `ObjectType::createObject()` or by calling the function `Object::copy()`.
//! They are also created implicitly by creating a variable of the type DPI_ORACLE_TYPE_OBJECT.
//! Objects are destroyed when the last reference is released by calling the function
//! `Object::release()`. All of the attributes of the structure `ODPIBaseType` are included in this
//! structure in addition to the ones specific to this structure described below.
use odpi::opaque::ODPIObject;

/// This structure represents instances of the types created by the SQL command CREATE OR REPLACE
/// TYPE
#[derive(Clone)]
pub struct Object {
    /// The ODPI-C Object pointer.
    pub inner: *mut ODPIObject,
}

impl Object {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIObject {
        self.inner
    }
}

impl From<*mut ODPIObject> for Object {
    fn from(inner: *mut ODPIObject) -> Object {
        Object { inner: inner }
    }
}
