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
use common::encoding;
use connection::Connection;
use context::Context;
use context::params::{CommonCreate, ConnCreate, PoolCreate};
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::{ODPIConn, ODPIPool};
use odpi::structs::ODPIEncodingInfo;
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

    /// Adds a reference to the pool. This is intended for situations where a reference to the pool
    /// needs to be maintained independently of the reference returned when the pool was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiPool_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_addRef".to_string()))
    }

    /// Closes the pool and makes it unusable for further activity.
    ///
    /// * `close_mode` - one or more of the values from the enumeration `ODPIPoolCloseMode`, OR'ed
    /// together.
    pub fn close(&self, close_mode: flags::ODPIPoolCloseMode) -> Result<()> {
        try_dpi!(externs::dpiPool_close(self.inner, close_mode),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_close".to_string()))
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

    /// Returns the number of sessions in the pool that are busy.
    pub fn get_busy_count(&self) -> Result<u32> {
        let mut busy_count = 0;

        try_dpi!(externs::dpiPool_getBusyCount(self.inner, &mut busy_count),
                 Ok(busy_count),
                 ErrorKind::Pool("dpiPool_getBusyCount".to_string()))
    }

    /// Returns the encoding information used by the pool. This will be equivalent to the values
    /// passed when the pool was created, or the values retrieved from the environment variables
    /// NLS_LANG and NLS_NCHAR.
    pub fn get_encoding_info(&self) -> Result<encoding::Info> {
        let mut enc_info: ODPIEncodingInfo = Default::default();

        try_dpi!(externs::dpiPool_getEncodingInfo(self.inner, &mut enc_info),
                 Ok(enc_info.into()),
                 ErrorKind::Pool("dpiPool_getEncodingInfo".to_string()))
    }

    /// Returns the mode used for acquiring or getting connections from the pool.
    pub fn get_get_mode(&self) -> Result<flags::ODPIPoolGetMode> {
        let mut get_mode = flags::ODPIPoolGetMode::NoWait;

        try_dpi!(externs::dpiPool_getGetMode(self.inner, &mut get_mode),
                 Ok(get_mode),
                 ErrorKind::Pool("dpiPool_getGetMode".to_string()))
    }

    /// Returns the maximum lifetime of all sessions in the pool, in seconds. Sessions in the pool
    /// are terminated when this value has been reached, but only when another session is released
    /// back to the pool.
    pub fn get_max_lifetime_session(&self) -> Result<u32> {
        let mut max_lifetime_session = 0;

        try_dpi!(externs::dpiPool_getMaxLifetimeSession(self.inner, &mut max_lifetime_session),
                 Ok(max_lifetime_session),
                 ErrorKind::Pool("dpiPool_getMaxLifetimeSession".to_string()))
    }

    /// Returns the number of sessions in the pool that are open.
    pub fn get_open_count(&self) -> Result<u32> {
        let mut open_count = 0;

        try_dpi!(externs::dpiPool_getOpenCount(self.inner, &mut open_count),
                 Ok(open_count),
                 ErrorKind::Pool("dpiPool_getOpenCount".to_string()))
    }

    /// Returns the default size of the statement cache for sessions in the pool, in number of
    /// statements.
    pub fn get_stmt_cache_size(&self) -> Result<u32> {
        let mut stmt_cache_size = 0;

        try_dpi!(externs::dpiPool_getStmtCacheSize(self.inner, &mut stmt_cache_size),
                 Ok(stmt_cache_size),
                 ErrorKind::Pool("dpiPool_getStmtCacheSize".to_string()))
    }

    /// Returns the amount of time, in seconds, after which idle sessions in the pool are
    /// terminated, but only when another session is released back to the pool.
    pub fn get_timeout(&self) -> Result<u32> {
        let mut timeout = 0;

        try_dpi!(externs::dpiPool_getTimeout(self.inner, &mut timeout),
                 Ok(timeout),
                 ErrorKind::Pool("dpiPool_getTimeout".to_string()))
    }

    /// Releases a reference to the pool. A count of the references to the pool is maintained and
    /// when this count reaches zero, the memory associated with the pool is freed and the session
    /// pool is closed if that has not already taken place using the function `Pool::close()`.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiPool_release(self.inner),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_release".to_string()))
    }

    /// Sets the mode used for acquiring or getting connections from the pool.
    ///
    /// * `get_mode` - A value from the `ODPIGetMode` enumeration.
    pub fn set_get_mode(&self, get_mode: flags::ODPIPoolGetMode) -> Result<()> {
        try_dpi!(externs::dpiPool_setGetMode(self.inner, get_mode),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_setGetMode".to_string()))
    }

    /// Sets the maximum lifetime of all sessions in the pool, in seconds. Sessions in the pool are
    /// terminated when this value has been reached, but only when another session is released back
    /// to the pool.
    ///
    /// * `max_lifetime` - the maximum lifetime of all sessions in the pool, in seconds.
    pub fn set_max_lifetime_session(&self, max_lifetime: u32) -> Result<()> {
        try_dpi!(externs::dpiPool_setMaxLifetimeSession(self.inner, max_lifetime),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_setMaxLifetimeSession".to_string()))
    }

    /// Sets the default size of the statement cache for sessions in the pool.
    ///
    /// * `stmt_cache_size` - the new size of the statement cache, in number of statements.
    pub fn set_stmt_cache_size(&self, stmt_cache_size: u32) -> Result<()> {
        try_dpi!(externs::dpiPool_setStmtCacheSize(self.inner, stmt_cache_size),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_setStmtCacheSize".to_string()))
    }

    /// Sets the amount of time, in seconds, after which idle sessions in the pool are terminated,
    /// but only when another session is released back to the pool.
    pub fn set_timeout(&self, timeout: u32) -> Result<()> {
        try_dpi!(externs::dpiPool_setTimeout(self.inner, timeout),
                 Ok(()),
                 ErrorKind::Pool("dpiPool_setTimeout".to_string()))
    }
}

impl From<*mut ODPIPool> for Pool {
    fn from(inner: *mut ODPIPool) -> Pool {
        Pool { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use test::CREDS;
    use context::Context;
    use data::Data;
    use error::Result;
    use odpi::flags::{self, ODPIConnCloseMode, ODPIPoolCloseMode};
    use odpi::flags::ODPINativeTypeNum::*;
    use pool::Pool;
    use std::ffi::CString;

    fn pool_res() -> Result<()> {
        let ctxt = Context::create()?;

        let mut ccp = ctxt.init_common_create_params()?;
        let enc_cstr = CString::new("UTF-8").expect("badness");
        ccp.set_encoding(enc_cstr.as_ptr());
        ccp.set_nchar_encoding(enc_cstr.as_ptr());

        let pool = Pool::create(&ctxt,
                                Some(&CREDS[0]),
                                Some(&CREDS[1]),
                                Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                Some(ccp),
                                None)?;
        pool.add_ref()?;

        let ei = pool.get_encoding_info()?;
        assert_eq!(ei.encoding(), "UTF-8");
        assert_eq!(ei.nchar_encoding(), "UTF-8");
        assert_eq!(ei.max_bytes_per_char(), 4);
        assert_eq!(ei.max_bytes_per_nchar(), 4);

        let mut get_mode = pool.get_get_mode()?;
        assert_eq!(get_mode, flags::ODPIPoolGetMode::NoWait);
        pool.set_get_mode(flags::ODPIPoolGetMode::ForceGet)?;
        get_mode = pool.get_get_mode()?;
        assert_eq!(get_mode, flags::ODPIPoolGetMode::ForceGet);

        let mut max_lifetime_session = pool.get_max_lifetime_session()?;
        assert_eq!(max_lifetime_session, 0);
        pool.set_max_lifetime_session(3600)?;
        max_lifetime_session = pool.get_max_lifetime_session()?;
        assert_eq!(max_lifetime_session, 3600);

        let mut stmt_cache_size = pool.get_stmt_cache_size()?;
        assert_eq!(stmt_cache_size, 20);
        pool.set_stmt_cache_size(100)?;
        stmt_cache_size = pool.get_stmt_cache_size()?;
        assert_eq!(stmt_cache_size, 100);

        let mut timeout = pool.get_timeout()?;
        assert_eq!(timeout, 0);
        pool.set_timeout(3600)?;
        timeout = pool.get_timeout()?;
        assert_eq!(timeout, 3600);

        let conn = pool.acquire_connection(None, None, None)?;
        conn.add_ref()?;

        let version_info = conn.get_server_version()?;
        assert_eq!(version_info.version(), "12.1.0.2.0");
        assert_eq!(version_info.version_num(), 1201000200);
        assert_eq!(version_info.release(),
                   "Oracle Database 12c Standard Edition Release 12.1.0.2.0 - \
                   64bit Production");

        let stmt = conn.prepare_stmt(Some("select * from username where username = 'jozias'"),
                                     None,
                                     false)?;

        stmt.execute(flags::EXEC_DEFAULT)?;
        stmt.fetch()?;
        let (id_type, id_ptr) = stmt.get_query_value(1)?;
        let (username_type, username_ptr) = stmt.get_query_value(2)?;

        assert_eq!(id_type, Double);
        let id_data: Data = id_ptr.into();
        assert_eq!(id_data.as_double(), 1.0);

        assert_eq!(username_type, Bytes);
        let username_data: Data = username_ptr.into();
        assert_eq!(username_data.as_string(), "jozias");

        let busy_count = pool.get_busy_count()?;
        assert_eq!(busy_count, 1);

        let open_count = pool.get_open_count()?;
        assert_eq!(open_count, 1);

        conn.release()?;
        conn.close(ODPIConnCloseMode::DefaultClose, None)?;
        pool.release()?;
        pool.close(ODPIPoolCloseMode::DefaultClose)?;

        Ok(())
    }

    #[test]
    fn pool() {
        use std::io::{self, Write};

        match pool_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
