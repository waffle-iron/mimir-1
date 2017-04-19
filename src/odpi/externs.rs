//! ODPI-C externs
use ffi;
use libc::{c_int, c_uint};
use odpi::{opaque, structs};

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
}
