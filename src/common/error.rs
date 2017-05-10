// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! This structure is used for transferring error information from ODPI-C.
use odpi::structs::ODPIErrorInfo;
use std::ffi::CStr;
use std::{fmt, slice};

/// This structure is used for transferring error information from ODPI-C. All of the strings
/// referenced here may become invalid as soon as the next ODPI-C call is made.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Info {
    /// The OCI error code if an OCI error has taken place. If no OCI error has taken place the
    /// value is 0.
    code: i32,
    /// The parse error offset (in bytes) when executing a statement or the row offset when
    /// fetching batch error information. If neither of these cases are true, the value is 0.
    offset: u16,
    /// The error message as a string.
    message: String,
    /// The public ODPI-C function name which was called in which the error took place.
    fn_name: String,
    /// The internal action that was being performed when the error took place.
    action: String,
    /// The SQLSTATE code associated with the error.
    sql_state: String,
    /// A boolean value indicating if the error is recoverable. This member always has a false value
    /// unless both client and server are at release 12.1 or higher.
    recoverable: bool,
}

impl Info {
    /// Create a new `Info` struct.
    pub fn new(code: i32,
               offset: u16,
               message: String,
               fn_name: String,
               action: String,
               sql_state: String,
               recoverable: bool)
               -> Info {
        Info {
            code: code,
            offset: offset,
            message: message,
            fn_name: fn_name,
            action: action,
            sql_state: sql_state,
            recoverable: recoverable,
        }
    }

    /// Get the `code` value.
    pub fn code(&self) -> i32 {
        self.code
    }

    /// Get the `offset` value.
    pub fn offset(&self) -> u16 {
        self.offset
    }

    /// Get the `message` value.
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Get the `fn_name` value.
    pub fn fn_name(&self) -> &str {
        &self.fn_name
    }

    /// Get the `action` value.
    pub fn action(&self) -> &str {
        &self.action
    }

    /// Get the `sql_state` value.
    pub fn sql_state(&self) -> &str {
        &self.sql_state
    }

    /// Get the `recoverable` value.
    pub fn recoverable(&self) -> bool {
        self.recoverable
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f,
                 "{}: {}\nfn: {}\naction: {}\nsql_state: {}\nrecoverable: {}",
                 self.code,
                 self.message,
                 self.fn_name,
                 self.action,
                 self.sql_state,
                 self.recoverable)
    }
}

impl From<ODPIErrorInfo> for Info {
    fn from(err: ODPIErrorInfo) -> Info {
        let slice =
            unsafe { slice::from_raw_parts(err.message as *mut u8, err.message_length as usize) };
        let fn_name = unsafe { CStr::from_ptr(err.fn_name) }
            .to_string_lossy()
            .into_owned();
        let action = unsafe { CStr::from_ptr(err.action) }
            .to_string_lossy()
            .into_owned();
        let sql_state = unsafe { CStr::from_ptr(err.sql_state) }
            .to_string_lossy()
            .into_owned();
        Info::new(err.code,
                  err.offset,
                  String::from_utf8_lossy(slice).into_owned(),
                  fn_name,
                  action,
                  sql_state,
                  err.is_recoverable.is_positive())
    }
}
