//! ODPI-C externs
use ffi;
use libc::{c_int, c_uint, c_void};
use odpi::{opaque, structs};

/// The optional function pointer use in the `ODPISubscrCreateParams` struct.
pub type ODPISubscrCallback =
    Option<unsafe extern "C" fn(context: *mut c_void,
                                message: *mut structs::ODPISubscrMessage)>;

extern "C" {
    pub fn dpiContext_create(majorVersion: c_uint,
                             minorVersion: c_uint,
                             context: *mut *mut opaque::ODPIContext,
                             errorInfo: *mut ffi::ODPIErrorInfo)
                             -> c_int;
    pub fn dpiContext_destroy(context: *mut opaque::ODPIContext) -> c_int;
    pub fn dpiContext_getError(context: *const opaque::ODPIContext,
                               errorInfo: *mut ffi::ODPIErrorInfo);
    pub fn dpiContext_getClientVersion(context: *const opaque::ODPIContext,
                                       versionInfo: *mut ffi::ODPIVersionInfo)
                                       -> c_int;
    pub fn dpiContext_initCommonCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPICommonCreateParams)
                                             -> c_int;
    pub fn dpiContext_initConnCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIConnCreateParams)
                                           -> c_int;
    pub fn dpiContext_initPoolCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIPoolCreateParams)
                                           -> c_int;
    pub fn dpiContext_initSubscrCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPISubscrCreateParams)
                                             -> c_int;
}
