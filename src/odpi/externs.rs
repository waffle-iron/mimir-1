//! ODPI-C externs
use odpi::{flags, opaque, structs};

/// The optional function pointer use in the `ODPISubscrCreateParams` struct.
pub type ODPISubscrCallback =
    Option<unsafe extern "C" fn(context: *mut ::std::os::raw::c_void,
                                message: *mut structs::ODPISubscrMessage)>;

extern "C" {
    pub fn dpiContext_create(majorVersion: ::std::os::raw::c_uint,
                             minorVersion: ::std::os::raw::c_uint,
                             context: *mut *mut opaque::ODPIContext,
                             errorInfo: *mut structs::ODPIErrorInfo)
                             -> ::std::os::raw::c_int;
    pub fn dpiContext_destroy(context: *mut opaque::ODPIContext) -> ::std::os::raw::c_int;
    pub fn dpiContext_getClientVersion(context: *const opaque::ODPIContext,
                                       versionInfo: *mut structs::ODPIVersionInfo)
                                       -> ::std::os::raw::c_int;
    pub fn dpiContext_getError(context: *const opaque::ODPIContext,
                               errorInfo: *mut structs::ODPIErrorInfo);
    pub fn dpiContext_initCommonCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPICommonCreateParams)
                                             -> ::std::os::raw::c_int;
    pub fn dpiContext_initConnCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIConnCreateParams)
                                           -> ::std::os::raw::c_int;
    pub fn dpiContext_initPoolCreateParams(context: *const opaque::ODPIContext,
                                           params: *mut structs::ODPIPoolCreateParams)
                                           -> ::std::os::raw::c_int;
    pub fn dpiContext_initSubscrCreateParams(context: *const opaque::ODPIContext,
                                             params: *mut structs::ODPISubscrCreateParams)
                                             -> ::std::os::raw::c_int;
}

extern "C" {
    pub fn dpiConn_addRef(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_beginDistribTrans(conn: *mut opaque::ODPIConn,
                                     formatId: ::std::os::raw::c_long,
                                     transactionId: *const ::std::os::raw::c_char,
                                     transactionIdLength: u32,
                                     branchId: *const ::std::os::raw::c_char,
                                     branchIdLength: u32)
                                     -> ::std::os::raw::c_int;
    pub fn dpiConn_breakExecution(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_changePassword(conn: *mut opaque::ODPIConn,
                                  userName: *const ::std::os::raw::c_char,
                                  userNameLength: u32,
                                  oldPassword: *const ::std::os::raw::c_char,
                                  oldPasswordLength: u32,
                                  newPassword: *const ::std::os::raw::c_char,
                                  newPasswordLength: u32)
                                  -> ::std::os::raw::c_int;
    pub fn dpiConn_close(conn: *mut opaque::ODPIConn,
                         mode: flags::ODPIConnCloseMode,
                         tag: *const ::std::os::raw::c_char,
                         tagLength: u32)
                         -> ::std::os::raw::c_int;
    pub fn dpiConn_commit(conn: *mut opaque::ODPIConn) -> ::std::os::raw::c_int;
    pub fn dpiConn_create(context: *const opaque::ODPIContext,
                          userName: *const ::std::os::raw::c_char,
                          userNameLength: u32,
                          password: *const ::std::os::raw::c_char,
                          passwordLength: u32,
                          connectString: *const ::std::os::raw::c_char,
                          connectStringLength: u32,
                          commonParams: *const structs::ODPICommonCreateParams,
                          createParams: *mut structs::ODPIConnCreateParams,
                          conn: *mut *mut opaque::ODPIConn)
                          -> ::std::os::raw::c_int;
    pub fn dpiConn_deqObject(conn: *mut opaque::ODPIConn,
                             queueName: *const ::std::os::raw::c_char,
                             queueNameLength: u32,
                             options: *mut opaque::ODPIDeqOptions,
                             props: *mut opaque::ODPIMsgProps,
                             payload: *mut opaque::ODPIObject,
                             msgId: *mut *const ::std::os::raw::c_char,
                             msgIdLength: *mut u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_enqObject(conn: *mut opaque::ODPIConn,
                             queueName: *const ::std::os::raw::c_char,
                             queueNameLength: u32,
                             options: *mut opaque::ODPIEnqOptions,
                             props: *mut opaque::ODPIMsgProps,
                             payload: *mut opaque::ODPIObject,
                             msgId: *mut *const ::std::os::raw::c_char,
                             msgIdLength: *mut u32)
                             -> ::std::os::raw::c_int;
    pub fn dpiConn_getCurrentSchema(conn: *mut opaque::ODPIConn,
                                    value: *mut *const ::std::os::raw::c_char,
                                    valueLength: *mut u32)
                                    -> ::std::os::raw::c_int;
    pub fn dpiConn_getEdition(conn: *mut opaque::ODPIConn,
                              value: *mut *const ::std::os::raw::c_char,
                              valueLength: *mut u32)
                              -> ::std::os::raw::c_int;
    pub fn dpiConn_getEncodingInfo(conn: *mut opaque::ODPIConn,
                                   info: *mut structs::ODPIEncodingInfo)
     -> ::std::os::raw::c_int;
    pub fn dpiConn_getExternalName(conn: *mut opaque::ODPIConn,
                                   value: *mut *const ::std::os::raw::c_char,
                                   valueLength: *mut u32)
     -> ::std::os::raw::c_int;
    pub fn dpiConn_getServerVersion(conn: *mut opaque::ODPIConn,
                                    releaseString: *mut *const ::std::os::raw::c_char,
                                    releaseStringLength: *mut u32,
                                    versionInfo: *mut structs::ODPIVersionInfo)
                                    -> ::std::os::raw::c_int;
}
