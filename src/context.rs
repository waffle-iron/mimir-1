//! `oic` ODPI-C Context
use error::{ErrorKind, Result};
use ffi;
use odpi::{constants, flags, externs, opaque, structs};
use std::{env, ptr};
use std::ffi::CString;
use std::io::{self, Write};

/// ODPI-C Context Wrapper.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut opaque::ODPIContext,
    /// This structure is used for creating session pools and standalone connections to the
    /// database.
    common_create_params: structs::ODPICommonCreateParams,
    conn_create_params: structs::ODPIConnCreateParams,
    // pub pool_create_params: ffi::dpiPoolCreateParams,
    // pub subscr_create_params: ffi::dpiSubscrCreateParams,
}

impl Context {
    /// Create a new `Context` struct.
    pub fn new() -> Result<Context> {
        let mut err: ffi::ODPIErrorInfo = Default::default();
        let mut ctxt = ptr::null_mut();
        if unsafe {
               externs::dpiContext_create(constants::DPI_MAJOR_VERSION,
                                          constants::DPI_MINOR_VERSION,
                                          &mut ctxt,
                                          &mut err)
           } == constants::DPI_SUCCESS {
            let mut common_cp: structs::ODPICommonCreateParams = Default::default();
            let mut conn_cp: structs::ODPIConnCreateParams = Default::default();
            // let mut pool_create_params: ffi::dpiPoolCreateParams = Default::default();
            // let mut subscr_create_params: ffi::dpiSubscrCreateParams = Default::default();
            unsafe {
                externs::dpiContext_initCommonCreateParams(ctxt, &mut common_cp);
                externs::dpiContext_initConnCreateParams(ctxt, &mut conn_cp);
                // ffi::dpiContext_initPoolCreateParams(ctxt, &mut pool_create_params);
                // ffi::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_create_params);
            }
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
                //    pool_create_params: pool_create_params,
                //    subscr_create_params: subscr_create_params,
               })
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    /// Get the `context` value.
    pub fn context(&self) -> *mut opaque::ODPIContext {
        self.context
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

    /// Set the `auth_mode`
    ///
    /// Specifies the mode used for authorizing connections. It is expected to be one or more of the
    /// values from the enumeration `ODPIAuthMode`, OR'ed together. The default value is
    /// DPI_MODE_AUTH_DEFAULT.
    pub fn set_auth_mode(&mut self, auth_mode: flags::ODPIAuthMode) -> &mut Context {
        self.conn_create_params.auth_mode = auth_mode;
        self
    }

    /// Get the ODPI-C Version Information.
    pub fn version(&self) -> Result<String> {
        let mut version_info: ffi::ODPIVersionInfo = Default::default();
        try_dpi!(self,
                 externs::dpiContext_getClientVersion(self.context, &mut version_info));
        Ok(format!("{}.{}.{}.{}.{}",
                   version_info.version_num,
                   version_info.release_num,
                   version_info.update_num,
                   version_info.port_release_num,
                   version_info.port_update_num))
    }

    /// Get the ODPI-C Version Number.
    pub fn version_num(&self) -> Result<u32> {
        let mut version_info: ffi::ODPIVersionInfo = Default::default();
        try_dpi!(self,
                 externs::dpiContext_getClientVersion(self.context, &mut version_info));
        Ok(version_info.full_version_num)
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
                ctxt.set_encoding("UTF-8");
                ctxt.set_auth_mode(flags::DPI_MODE_AUTH_DEFAULT | flags::DPI_MODE_AUTH_SYSDBA);
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn version() {
        match Context::new() {
            Ok(ctxt) => {
                match ctxt.version() {
                    Ok(version) => {
                        assert!(version == "12.2.0.1.0");
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn version_num() {
        match Context::new() {
            Ok(ctxt) => {
                match ctxt.version_num() {
                    Ok(version_num) => {
                        println!("{}", version_num);
                        assert!(version_num == 1202000100);
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}