//! `oic` ODPI-C Context
use error::{ErrorKind, Result};
use odpi::{constants, flags, externs, opaque, structs};
use std::{env, ptr};
use std::ffi::CString;
use std::io::{self, Write};
use std::mem;

/// ODPI-C Context Wrapper.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut opaque::ODPIContext,
    /// This structure is used for creating session pools and standalone connections to the
    /// database.
    common_create_params: structs::ODPICommonCreateParams,
    /// This structure is used for creating connections to the database, whether standalone or
    /// acquired from a session pool.
    conn_create_params: structs::ODPIConnCreateParams,
    /// This structure is used for creating session pools, which can in turn be used to create
    /// connections that are acquired from that session pool.
    pool_create_params: structs::ODPIPoolCreateParams,
    /// This enumeration identifies the namespaces supported by subscriptions.
    subscr_create_params: structs::ODPISubscrCreateParams,
}

impl Context {
    /// Create a new `Context` struct.
    pub fn new() -> Result<Context> {
        let mut ctxt = ptr::null_mut();
        if unsafe {
               let mut err = mem::uninitialized::<structs::ODPIErrorInfo>();
               externs::dpiContext_create(constants::DPI_MAJOR_VERSION,
                                          constants::DPI_MINOR_VERSION,
                                          &mut ctxt,
                                          &mut err)
           } == constants::DPI_SUCCESS {
            unsafe {
                /// Setup references for the param structs.
                let mut common_cp: structs::ODPICommonCreateParams = mem::uninitialized();
                let mut conn_cp: structs::ODPIConnCreateParams = mem::uninitialized();
                let mut pool_cp: structs::ODPIPoolCreateParams = mem::uninitialized();
                let mut subscr_cp: structs::ODPISubscrCreateParams = mem::uninitialized();

                /// Initialize the structs with the default values.
                externs::dpiContext_initCommonCreateParams(ctxt, &mut common_cp);
                externs::dpiContext_initConnCreateParams(ctxt, &mut conn_cp);
                externs::dpiContext_initPoolCreateParams(ctxt, &mut pool_cp);
                externs::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_cp);

                /// Add the driver name to the Common Create Params struct.
                let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
                let driver_name_len = driver_name.len() as u32;
                let driver_name_cstr = CString::new(driver_name)?.as_ptr();

                common_cp.driver_name = driver_name_cstr;
                common_cp.driver_name_length = driver_name_len;

                Ok(Context {
                       context: ctxt,
                       common_create_params: common_cp,
                       conn_create_params: conn_cp,
                       pool_create_params: pool_cp,
                       subscr_create_params: subscr_cp,
                   })
            }
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    /// Get the `context` value.
    pub fn context(&self) -> *mut opaque::ODPIContext {
        self.context
    }

    /// Get the `create_mode` value.
    pub fn create_mode(&self) -> flags::ODPICreateMode {
        self.common_create_params.create_mode
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
                self.common_create_params.nchar_encoding = nchar_encoding_cstr.as_ptr();
            }
            Err(_e) => {
                writeln!(io::stderr(), "Unable to set nchar_encoding!").expect("badness");
            }
        }
        self
    }

    /// Get the `auth_mode` value.
    pub fn auth_mode(&self) -> flags::ODPIAuthMode {
        self.conn_create_params.auth_mode
    }

    /// Set the `auth_mode`
    ///
    /// Specifies the mode used for authorizing connections. It is expected to be one or more of the
    /// values from the enumeration `ODPIAuthMode`, OR'ed together. The default value is
    /// DPI_MODE_AUTH_DEFAULT.
    pub fn set_auth_mode(&mut self, auth_mode: flags::ODPIAuthMode) -> &mut Context {
        self.conn_create_params.auth_mode = auth_mode;
        self
    }

    /// Set the `min_sessions`
    ///
    /// Specifies the minimum number of sessions to be created by the session pool. This value is
    /// ignored if the dpiPoolCreateParams.homogeneous member has a value of 0. The default value is
    /// 1.
    pub fn set_min_sessions(&mut self, min_sessions: u32) -> &mut Context {
        self.pool_create_params.min_sessions = min_sessions;
        self
    }


    /// Set the `port_number`
    ///
    /// Specifies the port number on which to receive notifications. The default value is 0, which
    /// means that a port number will be selected by the Oracle client.
    pub fn set_port_number(&mut self, port_number: u32) -> &mut Context {
        self.subscr_create_params.port_number = port_number;
        self
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { externs::dpiContext_destroy(self.context) } == constants::DPI_FAILURE {
            use std::io::{self, Write};
            writeln!(io::stderr(), "Unable to destroy ODPI-C context!").expect("badness");
        }
    }
}

#[cfg(test)]
mod test {
    use Context;
    use flags;

    #[test]
    fn context() {
        match Context::new() {
            Ok(mut ctxt) => {
                assert!(ctxt.auth_mode() == flags::DPI_MODE_AUTH_DEFAULT);
                ctxt.set_encoding("UTF-8");
                ctxt.set_auth_mode(flags::DPI_MODE_AUTH_DEFAULT | flags::DPI_MODE_AUTH_SYSDBA);
                let create_mode = ctxt.create_mode();
                assert!(create_mode == flags::DPI_MODE_CREATE_DEFAULT);
            }
            Err(_e) => assert!(false),
        }
    }
}
