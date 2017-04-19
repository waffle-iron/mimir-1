//! `oic` ODPI-C Context
use libc::{c_int, c_uint};
use error::{ErrorKind, Result};
use ffi;
use odpi::init;
use std::{env, ptr};
use std::ffi::CString;
use std::io::{self, Write};

extern "C" {
    pub fn dpiContext_create(majorVersion: c_uint,
                             minorVersion: c_uint,
                             context: *mut *mut ODPI,
                             errorInfo: *mut ffi::ODPIErrorInfo)
                             -> c_int;
    pub fn dpiContext_destroy(context: *mut ODPI) -> c_int;
    pub fn dpiContext_getError(context: *const ODPI, errorInfo: *mut ffi::ODPIErrorInfo);
    pub fn dpiContext_getClientVersion(context: *const ODPI,
                                       versionInfo: *mut ffi::ODPIVersionInfo)
                                       -> c_int;
    pub fn dpiContext_initCommonCreateParams(context: *const ODPI,
                                             params: *mut init::ODPICommonCreateParams)
                                             -> c_int;
}

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ODPI([u8; 0]);

/// ODPI-C Context Wrapper.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut ODPI,
    /// This structure is used for creating session pools and standalone connections to the
    /// database.
    common_create_params: init::ODPICommonCreateParams,
    // conn_create_params: ffi::dpiConnCreateParams,
    // pub pool_create_params: ffi::dpiPoolCreateParams,
    // pub subscr_create_params: ffi::dpiSubscrCreateParams,
}

impl Context {
    /// Create a new `Context` struct.
    pub fn new() -> Result<Context> {
        let mut err: ffi::ODPIErrorInfo = Default::default();
        let mut ctxt = ptr::null_mut();
        if unsafe {
               dpiContext_create(ffi::DPI_MAJOR_VERSION,
                                 ffi::DPI_MINOR_VERSION,
                                 &mut ctxt,
                                 &mut err)
           } == ffi::DPI_SUCCESS {
            let mut ccp_ffi: init::ODPICommonCreateParams = Default::default();
            // let mut conn_create_params: ffi::dpiConnCreateParams = Default::default();
            // let mut pool_create_params: ffi::dpiPoolCreateParams = Default::default();
            // let mut subscr_create_params: ffi::dpiSubscrCreateParams = Default::default();
            unsafe {
                dpiContext_initCommonCreateParams(ctxt, &mut ccp_ffi);
                // ffi::dpiContext_initConnCreateParams(ctxt, &mut conn_create_params);
                // ffi::dpiContext_initPoolCreateParams(ctxt, &mut pool_create_params);
                // ffi::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_create_params);
            }
            let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
            let driver_name_len = driver_name.len() as u32;
            let driver_name_cstr = CString::new(driver_name)?.as_ptr();

            ccp_ffi.driver_name = driver_name_cstr;
            ccp_ffi.driver_name_length = driver_name_len;

            Ok(Context {
                   context: ctxt,
                   common_create_params: ccp_ffi,
                //    conn_create_params: conn_create_params,
                //    pool_create_params: pool_create_params,
                //    subscr_create_params: subscr_create_params,
               })
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    /// Get the `context` value.
    pub fn context(&self) -> *mut ODPI {
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

    /// Get the ODPI-C Version Information.
    pub fn version(&self) -> Result<String> {
        let mut version_info: ffi::ODPIVersionInfo = Default::default();
        try_dpi!(self,
                 dpiContext_getClientVersion(self.context, &mut version_info));
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
                 dpiContext_getClientVersion(self.context, &mut version_info));
        Ok(version_info.full_version_num)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { dpiContext_destroy(self.context) } == ffi::DPI_FAILURE {
            use std::io::{self, Write};
            writeln!(io::stderr(), "Unable to destroy ODPI-C context!").expect("badness");
        }
    }
}

#[cfg(test)]
mod test {
    use Context;

    #[test]
    fn context() {
        match Context::new() {
            Ok(mut ctxt) => {
                ctxt.set_encoding("UTF-8");
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
