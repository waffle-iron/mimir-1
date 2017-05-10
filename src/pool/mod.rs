// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Pool handles are used to represent session pools. They are created using the function `create()`
//! and can be closed by calling the function `close()` or releasing the last reference to the pool
//! by calling the function `release()`. Pools can be used to create connections by calling the
//! function `acquireConnection()`.
use odpi::opaque::ODPIPool;

/// This structure represents session pools and is available by handle to a calling application or '
/// driver.
pub struct Pool {
    /// An ODPI-C dpiPool opaque struct pointer.
    pool: *mut ODPIPool,
}

impl Pool {
    /// Create a new `Pool` struct from an ODPI-C opaque struct pointer.
    pub fn from_odpi(pool: *mut ODPIPool) -> Pool {
        Pool { pool: pool }
    }

    /// Get the `pool` value.
    pub fn get_pool(&self) -> *mut ODPIPool {
        self.pool
    }
}
