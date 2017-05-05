//! Connection handles are used to represent connections to the database. These can be standalone
//! connections created by calling the function `create()` or acquired from a session pool by
//! calling the function `acquireConnection()`. They can be closed by calling the function `close()`
//!  or releasing the last reference to the connection by calling the function `release()`.
//! Connection handles are used to create all handles other than session pools and context handles.
use context::Context;
use context::params::{CommonCreate, ConnCreate};
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIConn;
use slog::Logger;
use std::ptr;
use util::ODPIStr;

/// Connection handles are used to represent connections to the database.
#[allow(dead_code)]
pub struct Connection {
    /// The ODPI-C connection.
    conn: *mut ODPIConn,
    /// Optional stdout logger.
    stdout: Option<Logger>,
    /// Optoinal stderr logger.
    stderr: Option<Logger>,
}

impl Connection {
    /// Adds a reference to the connection. This is intended for situations where a reference to the
    /// connection needs to be maintained independently of the reference returned when the
    /// connection was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiConn_addRef(self.conn),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_addRef".to_string()))
    }

    /// Creates a standalone connection to a database or acquires a connection from a session pool
    /// and returns a reference to the connection.
    pub fn create(context: &Context,
                  username: Option<&str>,
                  password: Option<&str>,
                  connect_string: &str,
                  common_create_params: Option<CommonCreate>,
                  conn_create_params: Option<ConnCreate>)
                  -> Result<Connection> {
        let username_s = ODPIStr::from(username);
        let password_s = ODPIStr::from(password);
        let connect_string_s = ODPIStr::from(connect_string);
        let mut conn: *mut ODPIConn = ptr::null_mut();

        let comm_cp = if let Some(common_create_params) = common_create_params {
            common_create_params
        } else {
            context.init_common_create_params()?
        };

        let conn_cp = if let Some(conn_create_params) = conn_create_params {
            conn_create_params
        } else {
            context.init_conn_create_params()?
        };

        try_dpi!(externs::dpiConn_create(context.inner(),
                                         username_s.ptr(),
                                         username_s.len(),
                                         password_s.ptr(),
                                         password_s.len(),
                                         connect_string_s.ptr(),
                                         connect_string_s.len(),
                                         &comm_cp.inner(),
                                         &mut conn_cp.inner(),
                                         &mut conn),
                 Ok(Connection {
                        conn: conn,
                        stdout: None,
                        stderr: None,
                    }),
                 ErrorKind::Connection("dpiConn_create".to_string()))
    }

    /// Releases a reference to the connection. A count of the references to the connection is
    /// maintained and when this count reaches zero, the memory associated with the connection is
    /// freed and the connection is closed or released back to the session pool if that has not
    /// already taken place using the function `close()`.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiConn_release(self.conn),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_release".to_string()))
    }
}

#[cfg(test)]
mod test {
    use {ContextResult, CTXT, ENC};
    use error;
    use connection::Connection;

    enum ConnResult {
        Ok(Connection),
        Err(error::Error),
    }

    unsafe impl Sync for ConnResult {}

    lazy_static! {
        static ref CONN: ConnResult = {
            let ctxt = match *CTXT {
                ContextResult::Ok(ref ctxt) => ctxt,
                ContextResult::Err(ref _e) => return ConnResult::Err(
                    error::ErrorKind::Connection("CONTEXT".to_string()).into()
                ),
            };
            let ccp = match ctxt.init_common_create_params() {
                Ok(mut ccp) => {
                    ccp.set_encoding(ENC.as_ptr());
                    ccp.set_nchar_encoding(ENC.as_ptr());
                    ccp
                },
                Err(e) => return ConnResult::Err(e),
            };
            match Connection::create(ctxt,
                                     Some("jozias"),
                                     Some("chip18jj"),
                                     "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL",
                                     Some(ccp),
                                     None) {
                Ok(conn) => ConnResult::Ok(conn),
                Err(e) => ConnResult::Err(e),
            }
        };
    }

    #[test]
    fn create() {
        match *CONN {
            ConnResult::Ok(ref _conn) => assert!(true),
            ConnResult::Err(ref _e) => assert!(false),
        }
    }

    #[test]
    fn add_ref_release() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };

        match conn.add_ref() {
            Ok(_) => {
                match conn.release() {
                    Ok(_) => assert!(true),
                    Err(_) => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }
}
