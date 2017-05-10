// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! ODPI-C Constants
use std::os::raw::{c_int, c_uint};

/// Major version of ODPI-C
pub const DPI_MAJOR_VERSION: c_uint = 2;
/// Minor version of ODPI-C
pub const DPI_MINOR_VERSION: c_uint = 0;
/// Value returned on success.
pub const DPI_SUCCESS: c_int = 0;
/// Value returned on failure.
pub const DPI_FAILURE: c_int = -1;
