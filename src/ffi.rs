//! FFI bindings to ODPI-C.
#![allow(non_snake_case)]
use libc::{c_char, c_int, c_uint};
use std::ptr;

pub const DPI_MAJOR_VERSION: c_uint = 2;
pub const DPI_MINOR_VERSION: c_uint = 0;
pub const DPI_SUCCESS: c_int = 0;
pub const DPI_FAILURE: c_int = -1;

bitflags!{
    flags AuthMode: u32 {
        const DPI_MODE_AUTH_DEFAULT = 0b00000000,
        const DPI_MODE_AUTH_SYSDBA  = 0b00000010,
        const DPI_MODE_AUTH_SYSOPER = 0b00000100,
        const DPI_MODE_AUTH_PRELIM  = 0b00001000,
        const DPI_MODE_AUTH_SYSASM  = 0b1000000000000000,
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ODPIErrorInfo {
    pub code: i32,
    pub offset: u16,
    pub message: *const c_char,
    pub messageLength: u32,
    pub encoding: *const c_char,
    pub fnName: *const c_char,
    pub action: *const c_char,
    pub sqlState: *const c_char,
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
pub struct ODPIVersionInfo {
    pub version_num: c_int,
    pub release_num: c_int,
    pub update_num: c_int,
    pub port_release_num: c_int,
    pub port_update_num: c_int,
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

// impl Default for dpiConnCreateParams {
//     fn default() -> dpiConnCreateParams {
//         dpiConnCreateParams {
//             authMode: DPI_MODE_AUTH_DEFAULT,
//             connectionClass: ptr::null(),
//             connectionClassLength: 0,
//             purity: DPI_PURITY_DEFAULT,
//             newPassword: ptr::null(),
//             newPasswordLength: 0,
//             appContext: ptr::null_mut(),
//             numAppContext: 0,
//             externalAuth: 0,
//             externalHandle: ptr::null_mut(),
//             pool: ptr::null_mut(),
//             tag: ptr::null(),
//             tagLength: 0,
//             matchAnyTag: 0,
//             outTag: ptr::null(),
//             outTagLength: 0,
//             outTagFound: 0,
//         }
//     }
// }
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
