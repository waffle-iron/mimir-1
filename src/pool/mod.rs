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
use connection::Connection;
use context::Context;
use context::params::{CommonCreate, ConnCreate, PoolCreate};
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::{ODPIConn, ODPIPool};
use std::ptr;
use util::ODPIStr;

/// This structure represents session pools and is available by handle to a calling application or '
/// driver.
pub struct Pool {
    /// An ODPI-C dpiPool opaque struct pointer.
    inner: *mut ODPIPool,
}

impl Pool {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIPool {
        self.inner
    }

    /// Acquires a connection from the pool and returns a reference to it. This reference should be
    /// released as soon as it is no longer needed.
    ///
    /// * `username` - the name of the user used for authenticating the user, as a string in the
    /// encoding used for CHAR data. None is also acceptable if external authentication is being
    /// requested or credentials were supplied when the pool was created.
    /// * `password` - the password to use for authenticating the user, as a string in the encoding
    /// used for CHAR data. None is also acceptable if external authentication is being requested or
    /// if credentials were supplied when the pool was created.
    /// * `conn_create_params` - An optional `ConnCreate` structure which is used to specify
    /// parameters for connection creation. None is acceptable in which case all default parameters
    /// will be used when creating the connection.
    pub fn acquire_connection(&self,
                              username: Option<&str>,
                              password: Option<&str>,
                              conn_create_params: Option<ConnCreate>)
                              -> Result<Connection> {
        let username_s = ODPIStr::from(username);
        let password_s = ODPIStr::from(password);
        let conn_cp = if let Some(conn_create_params) = conn_create_params {
            conn_create_params
        } else {
            Default::default()
        };
        let mut conn: *mut ODPIConn = ptr::null_mut();

        try_dpi!(externs::dpiPool_acquireConnection(self.inner,
                                                    username_s.ptr(),
                                                    username_s.len(),
                                                    password_s.ptr(),
                                                    password_s.len(),
                                                    &mut conn_cp.inner(),
                                                    &mut conn),
                 Ok(conn.into()),
                 ErrorKind::Pool("dpiPool_acquireConnection".to_string()))
    }

    /// Creates a session pool which creates and maintains a group of stateless sessions to the
    /// database. The main benefit of session pooling is performance since making a connection to
    /// the database is a time-consuming activity, especially when the database is remote.
    ///
    /// * `context` - the context handle created earlier using the function `Context::create()`.
    /// * `username` - the name of the user used for authenticating sessions, as a string in the
    /// encoding used for CHAR data. None is also acceptable if external authentication is being
    /// requested or if a heterogeneous pool is being created.
    /// * `password` - the password to use for authenticating sessions, as a string in the encoding
    /// used for CHAR data. None is also acceptable if external authentication is being requested or
    ///  if a heterogeneous pool is being created.
    /// * `connect_string` - the connect string identifying the database to which connections are to
    /// be established by the session pool, as a string in the encoding used for CHAR data. None is
    /// also acceptable for local connections (identified by the environment variable ORACLE_SID).
    /// * `common_create_params` - a `CommonCreate` structure which is used to specify context
    /// parameters for pool creation. None is also acceptable in which case all default parameters
    /// will be used when creating the pool.
    /// * `conn_create_params` - a `PoolCreate` structure which is used to specify parameters for
    /// pool creation. None is also acceptable in which case all default parameters will be used for
    /// pool creation.
    pub fn create(context: &Context,
                  username: Option<&str>,
                  password: Option<&str>,
                  connect_string: Option<&str>,
                  common_create_params: Option<CommonCreate>,
                  pool_create_params: Option<PoolCreate>)
                  -> Result<Pool> {
        let username_s = ODPIStr::from(username);
        let password_s = ODPIStr::from(password);
        let connect_string_s = ODPIStr::from(connect_string);
        let mut inner: *mut ODPIPool = ptr::null_mut();

        let comm_cp = if let Some(common_create_params) = common_create_params {
            common_create_params
        } else {
            context.init_common_create_params()?
        };

        let pool_cp = if let Some(pool_create_params) = pool_create_params {
            pool_create_params
        } else {
            context.init_pool_create_params()?
        };

        try_dpi!(externs::dpiPool_create(context.inner(),
                                         username_s.ptr(),
                                         username_s.len(),
                                         password_s.ptr(),
                                         password_s.len(),
                                         connect_string_s.ptr(),
                                         connect_string_s.len(),
                                         &comm_cp.inner(),
                                         &mut pool_cp.inner(),
                                         &mut inner),
                 Ok(inner.into()),
                 ErrorKind::Pool("dpiPool_create".to_string()))
    }
}

impl From<*mut ODPIPool> for Pool {
    fn from(inner: *mut ODPIPool) -> Pool {
        Pool { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use error;
    use pool::Pool;
    use test::{ContextResult, CREDS, CTXT, ENC};

    enum PoolResult {
        Ok(Pool),
        Err(error::Error),
    }

    unsafe impl Sync for PoolResult {}

    lazy_static! {
        static ref POOL: PoolResult = {
            let ctxt = match *CTXT {
                ContextResult::Ok(ref ctxt) => ctxt,
                ContextResult::Err(ref _e) => return PoolResult::Err(
                    error::ErrorKind::Pool("CONTEXT".to_string()).into()
                ),
            };
            let ccp = match ctxt.init_common_create_params() {
                Ok(mut ccp) => {
                    ccp.set_encoding(ENC.as_ptr());
                    ccp.set_nchar_encoding(ENC.as_ptr());
                    ccp
                },
                Err(e) => return PoolResult::Err(e),
            };
            match Pool::create(ctxt,
                               Some(&CREDS[0]),
                               Some(&CREDS[1]),
                               Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                               Some(ccp),
                               None) {
                Ok(pool) => PoolResult::Ok(pool),
                Err(e) => PoolResult::Err(e),
            }
        };
    }
    #[test]
    fn create() {
        match *POOL {
            PoolResult::Ok(ref _pool) => assert!(true),
            PoolResult::Err(ref _e) => assert!(false),
        }
    }
}
