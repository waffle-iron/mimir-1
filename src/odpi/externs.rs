//! ODPI-C externs
use libc::{c_char, c_int, c_uint, c_void};
use odpi::{flags, opaque, structs};

/// The optional function pointer use in the `ODPISubscrCreateParams` struct.
pub type ODPISubscrCallback =
    Option<unsafe extern "C" fn(context: *mut c_void,
                                message: *mut structs::ODPISubscrMessage)>;

extern "C" {
    pub fn dpiContext_create(majorVersion: c_uint,
                             minorVersion: c_uint,
                             context: *mut *mut opaque::dpiContext,
                             errorInfo: *mut structs::ODPIErrorInfo)
                             -> c_int;
    pub fn dpiContext_destroy(context: *mut opaque::dpiContext) -> c_int;
    pub fn dpiContext_getError(context: *const opaque::dpiContext,
                               errorInfo: *mut structs::ODPIErrorInfo);
    pub fn dpiContext_getClientVersion(context: *const opaque::dpiContext,
                                       versionInfo: *mut structs::ODPIVersionInfo)
                                       -> c_int;
    pub fn dpiContext_initCommonCreateParams(context: *const opaque::dpiContext,
                                             params: *mut structs::dpiCommonCreateParams)
                                             -> c_int;
    pub fn dpiContext_initConnCreateParams(context: *const opaque::dpiContext,
                                           params: *mut structs::ODPIConnCreateParams)
                                           -> c_int;
    pub fn dpiContext_initPoolCreateParams(context: *const opaque::dpiContext,
                                           params: *mut structs::ODPIPoolCreateParams)
                                           -> c_int;
    pub fn dpiContext_initSubscrCreateParams(context: *const opaque::dpiContext,
                                             params: *mut structs::ODPISubscrCreateParams)
                                             -> c_int;
}

extern "C" {
    pub fn dpiConn_create(context: *const opaque::dpiContext,
                          userName: *const c_char,
                          userNameLength: u32,
                          password: *const c_char,
                          passwordLength: u32,
                          connectString: *const c_char,
                          connectStringLength: u32,
                          commonParams: *const structs::dpiCommonCreateParams,
                          createParams: *mut structs::ODPIConnCreateParams,
                          conn: *mut *mut opaque::ODPIConn)
                          -> c_int;
    pub fn dpiConn_close(conn: *mut opaque::ODPIConn,
                         mode: flags::ODPIConnCloseMode,
                         tag: *const c_char,
                         tagLength: u32)
                         -> c_int;
}
