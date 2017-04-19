//! `oci` errors
use odpi::context::Context;
use ffi;
use std::ffi::CStr;
use std::slice;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct DbError {
    code: i32,
    offset: u16,
    message: String,
    fn_name: String,
    action: String,
}

impl DbError {
    pub fn new(code: i32,
               offset: u16,
               message: String,
               fn_name: String,
               action: String)
               -> DbError {
        DbError {
            code: code,
            offset: offset,
            message: message,
            fn_name: fn_name,
            action: action,
        }
    }

    /// Oracle error code if OciError. always zero if DpiError
    pub fn code(&self) -> i32 {
        self.code
    }

    /// ? (used for Batch Errors?)
    pub fn offset(&self) -> u16 {
        self.offset
    }

    /// error message
    pub fn message(&self) -> &String {
        &self.message
    }

    /// function name in ODPI-C used by rust-oracle
    pub fn fn_name(&self) -> &String {
        &self.fn_name
    }

    /// action name in ODPI-C used by rust-oracle
    pub fn action(&self) -> &String {
        &self.action
    }
}

fn error_from_dpi_error(err: &ffi::dpiErrorInfo) -> ErrorKind {
    let err = DbError::new(err.code,
                           err.offset,
                           String::from_utf8_lossy(unsafe {
                                                       slice::from_raw_parts(err.message as *mut u8,
                                                                             err.messageLength as
                                                                             usize)
                                                   })
                                   .into_owned(),
                           unsafe { CStr::from_ptr(err.fnName) }
                               .to_string_lossy()
                               .into_owned(),
                           unsafe { CStr::from_ptr(err.action) }
                               .to_string_lossy()
                               .into_owned());
    if err.message().starts_with("DPI") {
        ErrorKind::DpiError(err)
    } else {
        ErrorKind::OciError(err)
    }
}

pub fn error_from_dpi_context(ctxt: &Context) -> ErrorKind {
    let mut err: ffi::dpiErrorInfo = Default::default();
    unsafe {
        ffi::dpiContext_getError(ctxt.context, &mut err);
    };
    error_from_dpi_error(&err)
}

error_chain! {
    foreign_links {
        Nul(::std::ffi::NulError);
        Var(::std::env::VarError);
    }

    errors {
        ContextCreateFailed {
            description("Failed to create the ODPI-C context!")
            display("Failed to create the ODPI-C context!")
        }
        DpiError(err: DbError) {
            description("Unable to determine the interface!")
            display("Unable to determine the interface!")
        }
        OciError(err: DbError) {
            description("Unable to determine the interface!")
            display("Unable to determine the interface!")
        }
    }
}
