//! FFI bindings to ODPI-C.
#![allow(non_snake_case)]
use libc::{c_char, c_int};
use std::ptr;

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// This structure is used for transferring error information from ODPI-C. All of the strings
/// referenced here may become invalid as soon as the next ODPI-C call is made.
pub struct ODPIErrorInfo {
    /// The OCI error code if an OCI error has taken place. If no OCI error has taken place the
    /// value is 0.
    pub code: i32,
    /// The parse error offset (in bytes) when executing a statement or the row offset when fetching
    /// batch error information. If neither of these cases are true, the value is 0.
    pub offset: u16,
    /// The error message as a byte string in the encoding specified by the dpiErrorInfo.encoding
    /// member.
    pub message: *const c_char,
    /// The length of the dpiErrorInfo.message member, in bytes.
    pub messageLength: u32,
    /// The encoding in which the error message is encoded as a null-terminated string. For OCI
    /// errors this is the CHAR encoding used when the connection was created. For ODPI-C specific
    /// errors this is UTF-8.
    pub encoding: *const c_char,
    /// The public ODPI-C function name which was called in which the error took place. This is a
    /// null-terminated ASCII string.
    pub fnName: *const c_char,
    /// The internal action that was being performed when the error took place. This is a
    /// null-terminated ASCII string.
    pub action: *const c_char,
    /// The SQLSTATE code associated with the error. This is a 5 character null-terminated string.
    pub sqlState: *const c_char,
    /// A boolean value indicating if the error is recoverable. This member always has a value of 0
    /// unless both client and server are at release 12.1 or higher.
    pub isRecoverable: c_int,
}

impl Default for ODPIErrorInfo {
    fn default() -> ODPIErrorInfo {
        ODPIErrorInfo {
            code: 0,
            offset: 0,
            message: ptr::null(),
            messageLength: 0,
            encoding: ptr::null(),
            fnName: ptr::null(),
            action: ptr::null(),
            sqlState: ptr::null(),
            isRecoverable: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for returning Oracle version information about the Oracle Client.
pub struct ODPIVersionInfo {
    /// Specifies the major version of the Oracle Client or Database.
    pub version_num: c_int,
    /// Specifies the release version of the Oracle Client or Database.
    pub release_num: c_int,
    /// Specifies the update version of the Oracle Client or Database.
    pub update_num: c_int,
    /// Specifies the port specific release version of the Oracle Client or Database.
    pub port_release_num: c_int,
    /// Specifies the port specific update version of the Oracle Client or Database.
    pub port_update_num: c_int,
    /// Specifies the full version (all five components) as a number that is suitable for
    /// comparison with the result of the macro DPI_ORACLE_VERSION_TO_NUMBER.
    pub full_version_num: u32,
}

impl Default for ODPIVersionInfo {
    fn default() -> ODPIVersionInfo {
        ODPIVersionInfo {
            version_num: 0,
            release_num: 0,
            update_num: 0,
            port_release_num: 0,
            port_update_num: 0,
            full_version_num: 0,
        }
    }
}
//
// impl Default for dpiPoolCreateParams {
//     fn default() -> dpiPoolCreateParams {
//         dpiPoolCreateParams {
//             minSessions: 0,
//             maxSessions: 0,
//             sessionIncrement: 0,
//             pingInterval: 0,
//             pingTimeout: 0,
//             homogeneous: 0,
//             externalAuth: 0,
//             getMode: DPI_MODE_POOL_GET_WAIT,
//             outPoolName: ptr::null(),
//             outPoolNameLength: 0,
//         }
//     }
// }
//
// impl Default for dpiSubscrCreateParams {
//     fn default() -> dpiSubscrCreateParams {
//         dpiSubscrCreateParams {
//             subscrNamespace: DPI_SUBSCR_NAMESPACE_DBCHANGE,
//             protocol: DPI_SUBSCR_PROTO_CALLBACK,
//             qos: DPI_SUBSCR_QOS_RELIABLE,
//             operations: DPI_OPCODE_ALL_OPS,
//             portNumber: 0,
//             timeout: 0,
//             name: ptr::null(),
//             nameLength: 0,
//             callback: None,
//             callbackContext: ptr::null_mut(),
//             recipientName: ptr::null(),
//             recipientNameLength: 0,
//         }
//     }
// }
