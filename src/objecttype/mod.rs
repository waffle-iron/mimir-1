//! Object type handles are used to represent types such as those created by the SQL command CREATE
//! OR REPLACE TYPE. They are created using the function `Connection::get_object_type()` or
//! implicitly when fetching from a column containing objects by calling the function
//! `Statement::get_query_info()`. Object types are also retrieved when used as attributes in
//! another object by calling the function `ObjectAttribute::get_info()` or as the element type of a
//! collection by calling the function `ObjectType::get_info()`. They are destroyed when the last
//! reference is released by calling the function `ObjectType::release()`.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIObjectType;

/// Object type handles are used to represent types such as those created by the SQL command CREATE
/// OR REPLACE TYPE.
pub struct ObjectType {
    /// A pointer to the opaque `ODPIObjectType`.
    inner: *mut ODPIObjectType,
}

impl ObjectType {
    /// Get the pointer to the inner ODPI struct.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIObjectType {
        self.inner
    }

    /// Adds a reference to the object type. This is intended for situations where a reference to
    /// the object type needs to be maintained independently of the reference returned when the
    /// object type was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiObjectType_addRef(self.inner),
                 Ok(()),
                 ErrorKind::ObjectType("dpiObjectType_addRef".to_string()))
    }
}

impl From<*mut ODPIObjectType> for ObjectType {
    fn from(oot: *mut ODPIObjectType) -> ObjectType {
        ObjectType { inner: oot }
    }
}
