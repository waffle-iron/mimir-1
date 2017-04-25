//! ODPI-C Constants
use std::os::raw::{c_int, c_uint};

/// Major version of ODPI-C
pub const DPI_MAJOR_VERSION: c_uint = 2;
/// Minor version of ODPI-C
pub const DPI_MINOR_VERSION: c_uint = 0;
/// Value returned on success.
pub const DPI_SUCCESS: c_int = 0;
// /// Value returned on failure.
// pub const DPI_FAILURE: c_int = -1;
