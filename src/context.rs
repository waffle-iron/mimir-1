//! `oic` ODPI-C Context
use libc::{c_char, uint32_t};
use error::{ErrorKind, Result};
use ffi::{DPI_FAILURE, DPI_MAJOR_VERSION, DPI_MINOR_VERSION, DPI_SUCCESS, dpiCommonCreateParams,
          dpiContext, dpiContext_create, dpiContext_destroy, dpiContext_initCommonCreateParams,
          dpiCreateMode, dpiErrorInfo};
use std::{env, ptr};
use std::ffi::CString;
use std::io::{self, Write};
use std::mem;

/// ODPI-C Context Wrapper.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut dpiContext,
    /// This structure is used for creating session pools and standalone connections to the
    /// database.
    common_create_params: dpiCommonCreateParams,
    // /// This structure is used for creating connections to the database, whether standalone or
    // /// acquired from a session pool.
    // conn_create_params: structs::ODPIConnCreateParams,
    // /// This structure is used for creating session pools, which can in turn be used to create
    // /// connections that are acquired from that session pool.
    // pool_create_params: structs::ODPIPoolCreateParams,
    // /// This enumeration identifies the namespaces supported by subscriptions.
    // subscr_create_params: structs::ODPISubscrCreateParams,
}

impl Context {
    /// Create a new `Context` struct.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation, cast_possible_wrap))]
    pub fn new() -> Result<Context> {
        let mut ctxt = ptr::null_mut();
        if unsafe {
               let mut err = mem::uninitialized::<dpiErrorInfo>();
               dpiContext_create(DPI_MAJOR_VERSION, DPI_MINOR_VERSION, &mut ctxt, &mut err)
           } == DPI_SUCCESS as i32 {
            unsafe {
                /// Setup references for the param structs.
                let mut common_cp = mem::uninitialized::<dpiCommonCreateParams>();
                // let mut conn_cp: structs::ODPIConnCreateParams = mem::uninitialized();
                // let mut pool_cp: structs::ODPIPoolCreateParams = mem::uninitialized();
                // let mut subscr_cp: structs::ODPISubscrCreateParams = mem::uninitialized();

                /// Initialize the structs with the default values.
                dpiContext_initCommonCreateParams(ctxt, &mut common_cp);
                // externs::dpiContext_initConnCreateParams(ctxt, &mut conn_cp);
                // externs::dpiContext_initPoolCreateParams(ctxt, &mut pool_cp);
                // externs::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_cp);

                /// Add the driver name to the Common Create Params struct.
                let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
                let driver_name_ptr = driver_name.as_ptr() as *const c_char;
                let driver_name_len = driver_name.len() as uint32_t;

                // common_cp.createMode |= flags::DPI_MODE_CREATE_THREADED;
                common_cp.driverName = driver_name_ptr;
                common_cp.driverNameLength = driver_name_len;

                Ok(Context {
                       context: ctxt,
                       common_create_params: common_cp,
                    //    conn_create_params: conn_cp,
                    //    pool_create_params: pool_cp,
                    //    subscr_create_params: subscr_cp,
                   })
            }
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    /// Get the `context` value.
    pub fn context(&self) -> *mut dpiContext {
        self.context
    }

    /// Get the `common_create_params` value.
    pub fn common_create_params(&self) -> dpiCommonCreateParams {
        self.common_create_params
    }

    /// Get the `conn_create_params` value.
    // pub fn conn_create_params(&self) -> structs::ODPIConnCreateParams {
    //     self.conn_create_params
    // }

    // Get the `create_mode` value.
    pub fn create_mode(&self) -> dpiCreateMode {
        self.common_create_params.createMode
    }

    /// Get the `driver_name` value.
    pub fn driver_name(&self) -> String {
        let vec = unsafe {
            ::std::slice::from_raw_parts(self.common_create_params.driverName as *mut u8,
                                         self.common_create_params.driverNameLength as usize)
        };
        String::from_utf8_lossy(vec).into_owned()
    }

    /// Set the `encoding`.
    ///
    /// Specifies the encoding to use for CHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_LANG environment variable. The default value is NULL.
    pub fn set_encoding(&mut self, encoding: &str) -> &mut Context {
        match CString::new(encoding) {
            Ok(encoding_cstr) => {
                self.common_create_params.encoding = encoding_cstr.as_ptr();
            }
            Err(_e) => {
                writeln!(io::stderr(), "Unable to set encoding!").expect("badness");
            }
        }
        self
    }

    /// Set the `nchar_encoding`.
    ///
    /// Specifies the encoding to use for NCHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_NCHAR environment variable. The default value is NULL.
    pub fn set_nchar_encoding(&mut self, nchar_encoding: &str) -> &mut Context {
        match CString::new(nchar_encoding) {
            Ok(nchar_encoding_cstr) => {
                self.common_create_params.nencoding = nchar_encoding_cstr.as_ptr();
            }
            Err(_e) => {
                writeln!(io::stderr(), "Unable to set nchar_encoding!").expect("badness");
            }
        }
        self
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { dpiContext_destroy(self.context) } == DPI_FAILURE {
            use std::io::{self, Write};
            writeln!(io::stderr(), "Unable to destroy ODPI-C context!").expect("badness");
        }
    }
}

#[cfg(test)]
mod test {
    use Context;
    use ffi::dpiCreateMode::*;

    #[test]
    fn context() {
        match Context::new() {
            Ok(mut ctxt) => {
                // assert!(ctxt.auth_mode() == DPI_MODE_AUTH_DEFAULT);
                ctxt.set_encoding("UTF-8");
                // ctxt.set_auth_mode(DPI_MODE_AUTH_DEFAULT | DPI_MODE_AUTH_SYSDBA);
                let create_mode = ctxt.create_mode();
                let driver_name = ctxt.driver_name();
                println!("{}", driver_name);
                assert!(create_mode == DPI_MODE_CREATE_DEFAULT);
                assert!(driver_name == "Rust Oracle: 0.1.0");
            }
            Err(_e) => assert!(false),
        }
    }
}
