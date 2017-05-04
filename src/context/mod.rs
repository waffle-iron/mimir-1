//! Context handles are the top level handles created by the library and are used for all error
//! handling as well as creating pools and standalone connections to the database. The first call to
//! ODPI-C by any application must be `dpiContext_create()` which will create the context as well as
//! validate the version used by the application. Context handles are destroyed by using the
//! function `dpiContext_destroy()`.
use error::{ErrorKind, Result};
use odpi::constants::{DPI_MAJOR_VERSION, DPI_MINOR_VERSION};
use odpi::externs;
use odpi::opaque::ODPIContext;
use odpi::structs::{ODPICommonCreateParams, ODPIErrorInfo, ODPIVersionInfo};
use public::VersionInfo;
use std::{mem, ptr};

pub mod errorinfo;
pub mod params;

use self::errorinfo::ErrorInfo;
use self::params::Create;

/// This structure represents the context in which all activity in the library takes place.
pub struct Context {
    /// This structure represents the context in which all activity in the library takes place.
    context: *mut ODPIContext,
    /// The common `Create` parameters.
    create_params: Create,
}

/// Initializes the `Create` structure to default values.
fn init_common_create_params(ctxt: *mut ODPIContext) -> Result<Create> {
    let mut ccp = unsafe { mem::uninitialized::<ODPICommonCreateParams>() };

    try_dpi!(externs::dpiContext_initCommonCreateParams(ctxt, &mut ccp),
             Ok(Create::new(&mut ccp)),
             ErrorKind::Context("dpiContext_initCommonCreateParams".to_string()))
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
                 {
                     let ccp = init_common_create_params(ctxt)?;
                     let context = Context {
                         context: ctxt,
                         create_params: ccp,
                     };
                     Ok(context)
                 },
                 ErrorKind::Context("dpiContext_create".to_string()))
    }

    /// Destroys the context that was earlier created with the function `create`.
    pub fn destroy(&self) -> Result<()> {
        try_dpi!(externs::dpiContext_destroy(self.context),
                 Ok(()),
                 ErrorKind::Context("dpiContext_destroy".to_string()))
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

    ///
    pub fn create_params(&self) -> &Create {
        &self.create_params
    }

    /// Get the common `Create` params.
    pub fn create_params_mut(&mut self) -> &mut Create {
        &mut self.create_params
    }
}

#[cfg(test)]
mod test {
    use super::Context;
    use odpi::flags;

    #[test]
    fn create() {
        match Context::create() {
            Ok(ref mut ctxt) => {
                let new_flags = ctxt.create_params().get_mode() | flags::DPI_MODE_CREATE_THREADED;
                ctxt.create_params_mut().set_mode(new_flags);
                assert!(true);
            }
            Err(_e) => assert!(false),
        }
    }
}
