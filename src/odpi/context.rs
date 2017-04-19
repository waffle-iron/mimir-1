//! `oic` ODPI-C Context
use error::{ErrorKind, Result};
use ffi;
use libc::{c_char, uint32_t};
use std::env;
use std::ffi::CString;
use std::ptr;

pub struct Context {
    pub context: *mut ffi::dpiContext,
    pub common_create_params: ffi::dpiCommonCreateParams,
    pub conn_create_params: ffi::dpiConnCreateParams,
    pub pool_create_params: ffi::dpiPoolCreateParams,
    pub subscr_create_params: ffi::dpiSubscrCreateParams,
}

impl Context {
    pub fn new() -> Result<Context> {
        let mut err: ffi::dpiErrorInfo = Default::default();
        let mut ctxt = ptr::null_mut();
        if unsafe {
               ffi::dpiContext_create(ffi::DPI_MAJOR_VERSION,
                                      ffi::DPI_MINOR_VERSION,
                                      &mut ctxt,
                                      &mut err)
           } == ffi::DPI_SUCCESS as i32 {
            let mut common_create_params: ffi::dpiCommonCreateParams = Default::default();
            let mut conn_create_params: ffi::dpiConnCreateParams = Default::default();
            let mut pool_create_params: ffi::dpiPoolCreateParams = Default::default();
            let mut subscr_create_params: ffi::dpiSubscrCreateParams = Default::default();
            unsafe {
                let utf8_ptr = CString::new("UTF-8")?.as_ptr() as *const c_char;
                let driver_name = format!("Rust Oracle: {}", env::var("CARGO_PKG_VERSION")?);
                let driver_name_len = driver_name.len() as uint32_t;
                let driver_name_cstr = CString::new(driver_name)?;
                ffi::dpiContext_initCommonCreateParams(ctxt, &mut common_create_params);
                ffi::dpiContext_initConnCreateParams(ctxt, &mut conn_create_params);
                ffi::dpiContext_initPoolCreateParams(ctxt, &mut pool_create_params);
                ffi::dpiContext_initSubscrCreateParams(ctxt, &mut subscr_create_params);
                common_create_params.createMode = ffi::DPI_MODE_CREATE_THREADED;
                common_create_params.encoding = utf8_ptr;
                common_create_params.nencoding = utf8_ptr;
                common_create_params.driverName = driver_name_cstr.as_ptr();
                common_create_params.driverNameLength = driver_name_len;
            }
            Ok(Context {
                   context: ctxt,
                   common_create_params: common_create_params,
                   conn_create_params: conn_create_params,
                   pool_create_params: pool_create_params,
                   subscr_create_params: subscr_create_params,
               })
        } else {
            Err(ErrorKind::ContextCreateFailed.into())
        }
    }

    pub fn version(&self) -> Result<String> {
        let mut version_info: ffi::dpiVersionInfo = Default::default();
        try_dpi!(self,
                 ffi::dpiContext_getClientVersion(self.context, &mut version_info));
        Ok(format!("{}.{}.{}.{}.{}",
                   version_info.versionNum,
                   version_info.releaseNum,
                   version_info.updateNum,
                   version_info.portReleaseNum,
                   version_info.portUpdateNum))
    }

    pub fn version_num(&self) -> Result<u32> {
        let mut version_info: ffi::dpiVersionInfo = Default::default();
        try_dpi!(self,
                 ffi::dpiContext_getClientVersion(self.context, &mut version_info));
        Ok(version_info.fullVersionNum)
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if unsafe { ffi::dpiContext_destroy(self.context) } == ffi::DPI_FAILURE {
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
            Ok(_ctxt) => {}
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
