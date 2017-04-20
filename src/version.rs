//! ODPI-C Version Information
use Context;
use error::{self, Result};
use odpi::{constants, externs, structs};
use std::mem;

/// Get the Oracle client version information.
pub fn client(context: &Context) -> Result<String> {
    unsafe {
        let mut version_info = mem::uninitialized::<structs::ODPIVersionInfo>();
        let res = externs::dpiContext_getClientVersion(context.context(), &mut version_info);

        if res == constants::DPI_SUCCESS {
            Ok(format!("{}.{}.{}.{}.{}",
                       version_info.version_num,
                       version_info.release_num,
                       version_info.update_num,
                       version_info.port_release_num,
                       version_info.port_update_num))
        } else {
            Err(error::from_dpi_context(context).into())
        }
    }
}

/// Get the Oracle client version number.
pub fn client_num(context: &Context) -> Result<u32> {
    unsafe {
        let mut version_info = mem::uninitialized::<structs::ODPIVersionInfo>();
        let res = externs::dpiContext_getClientVersion(context.context(), &mut version_info);

        if res == constants::DPI_SUCCESS {
            Ok(version_info.full_version_num)
        } else {
            Err(error::from_dpi_context(context).into())
        }
    }
}

#[cfg(test)]
mod test {
    use Context;
    use version;

    #[test]
    fn client() {
        match Context::new() {
            Ok(ref ctxt) => {
                match version::client(ctxt) {
                    Ok(version) => assert!(version == "12.2.0.1.0"),
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn client_num() {
        match Context::new() {
            Ok(ref ctxt) => {
                match version::client_num(ctxt) {
                    Ok(version_num) => assert!(version_num == 1202000100),
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
