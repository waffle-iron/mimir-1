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
