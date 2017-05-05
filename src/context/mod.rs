//! Context handles are the top level handles created by the library and are used for all error
//! handling as well as creating pools and standalone connections to the database. The first call to
//! ODPI-C by any application must be `dpiContext_create()` which will create the context as well as
//! validate the version used by the application. Context handles are destroyed by using the
//! function `dpiContext_destroy()`.
use error::{ErrorKind, Result};
use odpi::constants::{DPI_FAILURE, DPI_MAJOR_VERSION, DPI_MINOR_VERSION};
use odpi::externs;
use odpi::opaque::ODPIContext;
use odpi::structs::{ODPICommonCreateParams, ODPIConnCreateParams, ODPIErrorInfo, ODPIVersionInfo};
use public::VersionInfo;
use slog::Logger;
use std::{mem, ptr};

pub mod errorinfo;
pub mod params;

use self::errorinfo::ErrorInfo;
use self::params::{Conn, Create};
use std::env;
use util::ODPIStr;

/// This structure represents the context in which all activity in the library takes place.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut ODPIContext,
    /// Optional stdout logger.
    stdout: Option<Logger>,
    /// Optoinal stderr logger.
    stderr: Option<Logger>,
}

impl Context {
    /// Create a new `Context` struct.
    pub fn create() -> Result<Context> {
        let mut ctxt = ptr::null_mut();
        let mut err = unsafe { mem::uninitialized::<ODPIErrorInfo>() };

        try_dpi!(externs::dpiContext_create(DPI_MAJOR_VERSION,
                                            DPI_MINOR_VERSION,
                                            &mut ctxt,
                                            &mut err),
                 Ok(Context {
                        context: ctxt,
                        stdout: None,
                        stderr: None,
                    }),
                 ErrorKind::Context("dpiContext_create".to_string()))
    }

    /// Return information about the version of the Oracle Client that is being used.
    pub fn get_client_version(&self) -> Result<VersionInfo> {
        let mut version_info = unsafe { mem::uninitialized::<ODPIVersionInfo>() };
        try_dpi!(externs::dpiContext_getClientVersion(self.context, &mut version_info),
                 Ok(version_info.into()),
                 ErrorKind::Connection("dpiContext_getClientVersion".to_string()))
    }

    /// Returns error information for the last error that was raised by the library. This function
    /// must be called with the same thread that generated the error. It must also be called before
    /// any other ODPI-C library calls are made on the calling thread since the error information
    /// specific to that thread is cleared at the start of every ODPI-C function call.
    pub fn get_error(&self) -> ErrorInfo {
        unsafe {
            let mut error_info = mem::uninitialized::<ODPIErrorInfo>();
            externs::dpiContext_getError(self.context, &mut error_info);
            error_info.into()
        }
    }

    /// Initializes the `Create` structure to default values.
    pub fn init_common_create_params(&self) -> Result<Create> {
        let mut ccp = unsafe { mem::uninitialized::<ODPICommonCreateParams>() };

        try_dpi!(externs::dpiContext_initCommonCreateParams(self.context, &mut ccp),
                 {
                     let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                     let driver_name_s = ODPIStr::from(driver_name);
                     ccp.driver_name = driver_name_s.ptr();
                     ccp.driver_name_length = driver_name_s.len();
                     Ok(Create::new(ccp))
                 },
                 ErrorKind::Context("dpiContext_initCommonCreateParams".to_string()))
    }

    /// Initializes the `Conn` structure to default values.
    pub fn init_conn_create_params(&self) -> Result<Conn> {
        let mut conn = unsafe { mem::uninitialized::<ODPIConnCreateParams>() };

        try_dpi!(externs::dpiContext_initConnCreateParams(self.context, &mut conn),
                 {
                     Ok(Conn::new(conn))
                 },
                 ErrorKind::Context("dpiContext_initConnCreateParams".to_string()))
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { externs::dpiContext_destroy(self.context) } == DPI_FAILURE {
            try_error!(self.stderr, "Failed to destroy context");
        } else {
            try_info!(self.stdout, "Successfully destroyed context");
        }
    }
}

#[cfg(test)]
mod test {
    use super::Context;
    use super::params::AppContext;
    use odpi::flags;
    use std::ffi::CString;

    #[test]
    fn create() {
        match Context::create() {
            Ok(ref mut _ctxt) => assert!(true),
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn init_common_create_params() {
        match Context::create() {
            Ok(ref mut ctxt) => {
                match ctxt.init_common_create_params() {
                    Ok(ref mut ccp) => {
                        let default_flags = ccp.get_create_mode();
                        let new_flags = default_flags | flags::DPI_MODE_CREATE_THREADED;
                        let enc_cstr = CString::new("UTF-8").expect("badness");

                        ccp.set_create_mode(new_flags);
                        ccp.set_edition("1.0");
                        ccp.set_encoding(enc_cstr.as_ptr());
                        ccp.set_nchar_encoding(enc_cstr.as_ptr());

                        assert!(ccp.get_create_mode() ==
                                flags::DPI_MODE_CREATE_THREADED | flags::DPI_MODE_CREATE_DEFAULT);
                        assert!(ccp.get_encoding() == "UTF-8");
                        assert!(ccp.get_nchar_encoding() == "UTF-8");
                        assert!(ccp.get_edition() == "1.0");
                        assert!(ccp.get_driver_name() == "Rust Oracle: 0.1.0");
                    }
                    Err(_r) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn init_conn_create_params() {
        match Context::create() {
            Ok(ref mut ctxt) => {
                match ctxt.init_conn_create_params() {
                    Ok(ref mut conn) => {
                        let auth_default_flags = conn.get_auth_mode();
                        let auth_new_flags = auth_default_flags | flags::DPI_MODE_AUTH_SYSDBA;
                        let purity_default_flags = conn.get_purity();
                        let app_ctxt = AppContext::new("ns", "name", "value");
                        let mut app_ctxt_vec = Vec::new();
                        app_ctxt_vec.push(app_ctxt);

                        assert!(purity_default_flags == flags::DPI_PURITY_DEFAULT);

                        conn.set_auth_mode(auth_new_flags);
                        conn.set_connection_class("conn_class");
                        conn.set_purity(flags::DPI_PURITY_NEW);
                        conn.set_new_password("password");
                        conn.set_app_context(app_ctxt_vec);

                        let new_app_ctxt_vec = conn.get_app_context();

                        assert!(conn.get_auth_mode() ==
                                flags::DPI_MODE_AUTH_SYSDBA | flags::DPI_MODE_AUTH_DEFAULT);
                        assert!(conn.get_connection_class() == "conn_class");
                        assert!(conn.get_purity() == flags::DPI_PURITY_NEW);
                        assert!(conn.get_new_password() == "password");
                        assert!(conn.get_num_app_context() == 1);
                        assert!(new_app_ctxt_vec.len() == 1);
                    }
                    Err(_r) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
