//! Context handles are the top level handles created by the library and are used for all error
//! handling as well as creating pools and standalone connections to the database. The first call to
//! ODPI-C by any application must be `dpiContext_create()` which will create the context as well as
//! validate the version used by the application. Context handles are destroyed by using the
//! function `dpiContext_destroy()`.
use error::{ErrorKind, Result};
use odpi::constants::{DPI_FAILURE, DPI_MAJOR_VERSION, DPI_MINOR_VERSION};
use odpi::externs;
use odpi::opaque::ODPIContext;
use odpi::structs::{ODPICommonCreateParams, ODPIConnCreateParams, ODPIErrorInfo,
                    ODPIPoolCreateParams, ODPIVersionInfo};
use public::VersionInfo;
use slog::Logger;
use std::{mem, ptr};

pub mod errorinfo;
pub mod params;

use self::errorinfo::ErrorInfo;
use self::params::{CommonCreate, ConnCreate, PoolCreate};
use std::env;
use util::ODPIStr;

/// This structure represents the context in which all activity in the library takes place.
pub struct Context {
    /// A pointer the the ODPI-C dpiContext struct.
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

    /// Initializes the `CommonCreate` structure to default values.
    pub fn init_common_create_params(&self) -> Result<CommonCreate> {
        let mut ccp = unsafe { mem::uninitialized::<ODPICommonCreateParams>() };

        try_dpi!(externs::dpiContext_initCommonCreateParams(self.context, &mut ccp),
                 {
                     let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                     let driver_name_s = ODPIStr::from(driver_name);
                     ccp.driver_name = driver_name_s.ptr();
                     ccp.driver_name_length = driver_name_s.len();
                     Ok(CommonCreate::new(ccp))
                 },
                 ErrorKind::Context("dpiContext_initCommonCreateParams".to_string()))
    }

    /// Initializes the `Conn` structure to default values.
    pub fn init_conn_create_params(&self) -> Result<ConnCreate> {
        let mut conn = unsafe { mem::uninitialized::<ODPIConnCreateParams>() };

        try_dpi!(externs::dpiContext_initConnCreateParams(self.context, &mut conn),
                 {
                     Ok(ConnCreate::new(conn))
                 },
                 ErrorKind::Context("dpiContext_initConnCreateParams".to_string()))
    }

    /// Initializes the `Pool` structure to default values.
    pub fn init_pool_create_params(&self) -> Result<PoolCreate> {
        let mut pool = unsafe { mem::uninitialized::<ODPIPoolCreateParams>() };
        try_dpi!(externs::dpiContext_initPoolCreateParams(self.context, &mut pool),
                 Ok(PoolCreate::new(pool)),
                 ErrorKind::Context("dpiContext_initPoolCreateParams".to_string()))
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
                    Err(_e) => assert!(false),
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
                        let app_ctxt_1 = AppContext::new("ns", "name1", "value1");
                        let mut app_ctxt_vec = Vec::new();
                        app_ctxt_vec.push(app_ctxt);
                        app_ctxt_vec.push(app_ctxt_1);

                        assert!(purity_default_flags == flags::DPI_PURITY_DEFAULT);

                        conn.set_auth_mode(auth_new_flags);
                        conn.set_connection_class("conn_class");
                        conn.set_purity(flags::DPI_PURITY_NEW);
                        conn.set_new_password("password");
                        conn.set_app_context(app_ctxt_vec);
                        conn.set_external_auth(1);
                        conn.set_tag("you're it");
                        conn.set_match_any_tag(true);

                        let new_app_ctxt_vec = conn.get_app_context();

                        assert!(conn.get_auth_mode() ==
                                flags::DPI_MODE_AUTH_SYSDBA | flags::DPI_MODE_AUTH_DEFAULT);
                        assert!(conn.get_connection_class() == "conn_class");
                        assert!(conn.get_purity() == flags::DPI_PURITY_NEW);
                        assert!(conn.get_new_password() == "password");
                        assert!(conn.get_num_app_context() == 2);
                        assert!(new_app_ctxt_vec.len() == 2);

                        for (idx, ac) in new_app_ctxt_vec.iter().enumerate() {
                            assert!(ac.get_namespace_name() == "ns");
                            match idx {
                                0 => {
                                    assert!(ac.get_name() == "name");
                                    assert!(ac.get_value() == "value");
                                }
                                1 => {
                                    assert!(ac.get_name() == "name1");
                                    assert!(ac.get_value() == "value1");
                                }
                                _ => assert!(false),
                            }
                        }

                        assert!(conn.get_external_auth() == 1);
                        assert!(conn.get_tag() == "you're it");
                        assert!(conn.get_match_any_tag());
                        assert!(conn.get_out_tag() == "");
                        assert!(!conn.get_out_tag_found());
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn init_pool_create_params() {
        match Context::create() {
            Ok(ref mut ctxt) => {
                match ctxt.init_pool_create_params() {
                    Ok(ref mut pcp) => {
                        assert!(pcp.get_min_sessions() == 1);
                        assert!(pcp.get_max_sessions() == 1);
                        assert!(pcp.get_session_increment() == 0);
                        assert!(pcp.get_ping_interval() == 60);
                        assert!(pcp.get_ping_timeout() == 5000);
                        assert!(pcp.get_homogeneous());
                        assert!(!pcp.get_external_auth());
                        assert!(pcp.get_get_mode() == flags::DPI_MODE_POOL_GET_NOWAIT);
                        assert!(pcp.get_out_pool_name() == "");

                        pcp.set_min_sessions(10);
                        pcp.set_max_sessions(100);
                        pcp.set_session_increment(5);
                        pcp.set_ping_interval(-1);
                        pcp.set_ping_timeout(1000);
                        pcp.set_homogeneous(false);
                        pcp.set_external_auth(true);
                        pcp.set_get_mode(flags::DPI_MODE_POOL_GET_FORCEGET);

                        assert!(pcp.get_min_sessions() == 10);
                        assert!(pcp.get_max_sessions() == 100);
                        assert!(pcp.get_session_increment() == 5);
                        assert!(pcp.get_ping_interval() == -1);
                        assert!(pcp.get_ping_timeout() == 1000);
                        assert!(!pcp.get_homogeneous());
                        assert!(pcp.get_external_auth());
                        assert!(pcp.get_get_mode() == flags::DPI_MODE_POOL_GET_FORCEGET);
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
