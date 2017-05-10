// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! This structure is used for passing query metadata from ODPI-C.
use objecttype::ObjectType;
use odpi::flags;
use odpi::structs::ODPIQueryInfo;
use util::ODPIStr;

/// This structure is used for passing query metadata from ODPI-C. It is populated by the function
/// `Statement::get_query_info()`. All values remain valid as long as a reference is held to the
/// statement and the statement is not re-executed or closed.
pub struct Info {
    /// The ODPI-C query info struct.
    inner: ODPIQueryInfo,
}

impl Info {
    /// Create a new `Info` struct.
    pub fn new(inner: ODPIQueryInfo) -> Info {
        Info { inner: inner }
    }

    /// Get the `name` value.
    ///
    /// Specifies the name of the column which is being queried, as a string in the encoding used
    /// for CHAR data.
    pub fn name(&self) -> String {
        let name_s = ODPIStr::new(self.inner.name, self.inner.name_length);
        name_s.into()
    }

    /// Get the `oracle_type_num` value.
    ///
    /// Specifies the type of the column that is being queried. It will be one of the values from
    /// the enumeration `ODPIOracleTypeNum`.
    pub fn oracle_type_num(&self) -> flags::ODPIOracleTypeNum {
        self.inner.oracle_type_num
    }

    /// Get the `default_native_type_num` value.
    ///
    /// Specifies the default native type for the column that is being queried. It will be one of
    /// the values from the enumeration `ODPINativeTypeNum`.
    pub fn default_native_type_num(&self) -> flags::ODPINativeTypeNum {
        self.inner.default_native_type_num
    }

    /// Get the `db_size_in_bytes` value.
    ///
    /// Specifies the size in bytes (from the database's perspective) of the column that is being
    /// queried. This value is only populated for strings and binary columns. For all other columns
    /// the value is zero.
    pub fn db_size_in_bytes(&self) -> u32 {
        self.inner.db_size_in_bytes
    }

    /// Get the `client_size_in_bytes` value.
    ///
    /// Specifies the size in bytes (from the client's perspective) of the column that is being
    /// queried. This value is only populated for strings and binary columns. For all other columns
    /// the value is zero.
    pub fn client_size_in_bytes(&self) -> u32 {
        self.inner.client_size_in_bytes
    }

    /// Get the `size_in_chars` value.
    ///
    /// Specifies the size in characters of the column that is being queried. This value is only
    /// populated for string columns. For all other columns the value is zero.
    pub fn size_in_chars(&self) -> u32 {
        self.inner.size_in_chars
    }

    /// Get the `precision` value.
    ///
    /// Specifies the precision of the column that is being queried. This value is only populated
    /// for numeric and timestamp columns. For all other columns the value is zero.
    pub fn precision(&self) -> i16 {
        self.inner.precision
    }

    /// Get the `scale` value.
    ///
    /// Specifies the scale of the column that is being queried. This value is only populated for
    /// numeric columns. For all other columns the value is zero.
    pub fn scale(&self) -> i8 {
        self.inner.scale
    }

    /// Get the `null_ok' value.
    ///
    /// Specifies if the column that is being queried may return null values or not.
    pub fn null_ok(&self) -> bool {
        self.inner.null_ok == 1
    }

    /// Get the `object_type` value.
    ///
    /// Specifies a reference to the type of the object that is being queried. This value is only
    /// populated for named type columns. For all other columns the value is None.
    pub fn object_type(&self) -> Option<ObjectType> {
        if self.inner.object_type.is_null() {
            None
        } else {
            Some(self.inner.object_type.into())
        }
    }
}
