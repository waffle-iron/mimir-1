//! Context handles are the top level handles created by the library and are used for all error
//! handling as well as creating pools and standalone connections to the database. The first call to
//! ODPI-C by any application must be `new` which will create the context as well as validate the
//! version used by the application. Context handles are destroyed by using the function `destroy`.
use error::{ErrorKind, Result};
use odpi::constants::{DPI_FAILURE, DPI_SUCCESS, DPI_MAJOR_VERSION, DPI_MINOR_VERSION};
use odpi::externs;
use odpi::flags::{self, ODPIAuthMode, ODPICreateMode};
use odpi::opaque::ODPIContext;
use odpi::structs::{ODPICommonCreateParams, ODPIConnCreateParams, ODPIErrorInfo,
                    ODPIPoolCreateParams, ODPISubscrCreateParams};
use slog::Logger;
use std::{env, ptr};
use std::ffi::CStr;
use std::mem;
use std::os::raw::c_char;
use util::ODPIStr;

/// ODPI-C Context Wrapper.
#[derive(Clone)]
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut ODPIContext,
    /// This structure is used for creating session pools and standalone connections to the
    /// database.
    common_create_params: ODPICommonCreateParams,
    /// This structure is used for creating connections to the database, whether standalone or
    /// acquired from a session pool.
    conn_create_params: ODPIConnCreateParams,
    /// This structure is used for creating session pools, which can in turn be used to create
    /// connections that are acquired from that session pool.
    pool_create_params: ODPIPoolCreateParams,
    /// This enumeration identifies the namespaces supported by subscriptions.
    subscr_create_params: ODPISubscrCreateParams,
    /// Optional stdout logger.
    stdout: Option<Logger>,
    /// Optoinal stderr logger.
    stderr: Option<Logger>,
}

impl Context {
    /// Create a new `Context` struct.
    pub fn new() -> Result<Context> {
        let mut ctxt = ptr::null_mut();
        if unsafe {
               let mut err = mem::uninitialized::<ODPIErrorInfo>();
               externs::dpiContext_create(DPI_MAJOR_VERSION, DPI_MINOR_VERSION, &mut ctxt, &mut err)
           } == DPI_SUCCESS as i32 {
            unsafe {
                /// Setup references for the param structs.
                let mut common_cp = mem::uninitialized::<ODPICommonCreateParams>();
                let mut conn_cp = mem::uninitialized::<ODPIConnCreateParams>();
                let mut pool_cp = mem::uninitialized::<ODPIPoolCreateParams>();
                let mut subscr_cp = mem::uninitialized::<ODPISubscrCreateParams>();

                /// Initialize the structs with the default values.
                externs::dpiContext_initCommonCreateParams(ctxt, &mut common_cp);
                externs::dpiContext_initConnCreateParams(ctxt, &mut conn_cp);
                externs::dpiContext_initPoolCreateParams(ctxt, &mut pool_cp);
                externs::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_cp);

                /// Add the driver name to the Common Create Params struct.
                let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                let driver_name_s = ODPIStr::from(driver_name);

                common_cp.create_mode |= flags::DPI_MODE_CREATE_THREADED;
                common_cp.driver_name = driver_name_s.ptr();
                common_cp.driver_name_length = driver_name_s.len();

                Ok(Context {
                       context: ctxt,
                       common_create_params: common_cp,
                       conn_create_params: conn_cp,
                       pool_create_params: pool_cp,
                       subscr_create_params: subscr_cp,
                       stdout: None,
                       stderr: None,
                   })
            }
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    /// Set the `stdout` value.
    pub fn set_stdout(&mut self, stdout: Option<Logger>) -> &mut Context {
        self.stdout = stdout;
        self
    }

    /// Set the `stderr` value.
    pub fn set_stderr(&mut self, stderr: Option<Logger>) -> &mut Context {
        self.stderr = stderr;
        self
    }

    /// Get the `context` value.
    pub fn context(&self) -> *mut ODPIContext {
        self.context
    }

    /// Get the `common_create_params` value.
    pub fn common_create_params(&self) -> ODPICommonCreateParams {
        self.common_create_params
    }

    /// Get the `conn_create_params` value.
    pub fn conn_create_params(&self) -> ODPIConnCreateParams {
        self.conn_create_params
    }

    /// Get the `pool_create_params` value.
    pub fn pool_create_params(&self) -> ODPIPoolCreateParams {
        self.pool_create_params
    }

    /// Get the `subscr_create_params` value.
    pub fn subscr_create_params(&self) -> ODPISubscrCreateParams {
        self.subscr_create_params
    }

    /// Get the `create_mode` value.
    pub fn create_mode(&self) -> ODPICreateMode {
        self.common_create_params.create_mode
    }

    /// Get the `auth_mode` value.
    pub fn auth_mode(&self) -> ODPIAuthMode {
        self.conn_create_params.auth_mode
    }

    /// Set the `auth_mode` value.
    pub fn set_auth_mode(&mut self, auth_mode: ODPIAuthMode) -> &mut Context {
        self.conn_create_params.auth_mode = auth_mode;
        self
    }

    /// Get the `driver_name` value.
    pub fn driver_name(&self) -> String {
        let vec = unsafe {
            ::std::slice::from_raw_parts(self.common_create_params.driver_name as *mut u8,
                                         self.common_create_params.driver_name_length as usize)
        };
        String::from_utf8_lossy(vec).into_owned()
    }

    /// Get the `encoding` value.
    pub fn encoding(&self) -> String {
        unsafe {
            let encoding_cstr = CStr::from_ptr(self.common_create_params.encoding);
            encoding_cstr.to_string_lossy().into_owned()
        }
    }

    /// Set the `encoding`.
    ///
    /// Specifies the encoding to use for CHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_LANG environment variable. The default value is NULL.
    pub fn set_encoding(&mut self, encoding: &str) -> &mut Context {
        let mut nt_enc = String::from(encoding);
        nt_enc.push('\0');
        self.common_create_params.encoding = nt_enc.as_ptr() as *const c_char;
        self
    }

    /// Get the `nchar_encoding` value.
    pub fn nchar_encoding(&self) -> String {
        unsafe {
            let nchar_encoding_cstr = CStr::from_ptr(self.common_create_params.nchar_encoding);
            nchar_encoding_cstr.to_string_lossy().into_owned()
        }
    }

    /// Set the `nchar_encoding`.
    ///
    /// Specifies the encoding to use for NCHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_NCHAR environment variable. The default value is NULL.
    pub fn set_nchar_encoding(&mut self, nchar_encoding: &str) -> &mut Context {
        let mut nt_enc = String::from(nchar_encoding);
        nt_enc.push('\0');
        self.common_create_params.nchar_encoding = nt_enc.as_ptr() as *const c_char;
        self
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { externs::dpiContext_destroy(self.context()) } == DPI_FAILURE {
            try_error!(self.stderr, "Failed to destroy context");
        } else {
            try_info!(self.stdout, "Successfully destroyed context");
        }
    }
}

#[cfg(test)]
mod test {
    use Context;
    use odpi::flags::{DPI_MODE_AUTH_DEFAULT, DPI_MODE_AUTH_SYSDBA, DPI_MODE_CREATE_DEFAULT,
                      DPI_MODE_CREATE_THREADED};

    #[test]
    fn context() {
        match Context::new() {
            Ok(mut ctxt) => {
                assert!(ctxt.auth_mode() == DPI_MODE_AUTH_DEFAULT);
                let enc = "UTF-8";
                ctxt.set_encoding(enc);
                ctxt.set_nchar_encoding(enc);
                ctxt.set_auth_mode(DPI_MODE_AUTH_DEFAULT | DPI_MODE_AUTH_SYSDBA);
                let create_mode = ctxt.create_mode();
                let driver_name = ctxt.driver_name();
                let encoding = ctxt.encoding();
                let nchar_encoding = ctxt.nchar_encoding();
                let auth_mode = ctxt.auth_mode();
                assert!(auth_mode == DPI_MODE_AUTH_DEFAULT | DPI_MODE_AUTH_SYSDBA);
                assert!(create_mode == DPI_MODE_CREATE_DEFAULT | DPI_MODE_CREATE_THREADED);
                assert!(driver_name == "Rust Oracle: 0.1.0");
                assert!(encoding == "UTF-8");
                assert!(nchar_encoding == "UTF-8");
            }
            Err(_e) => assert!(false),
        }
    }
}
