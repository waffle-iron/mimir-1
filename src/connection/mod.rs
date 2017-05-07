//! Connection handles are used to represent connections to the database. These can be standalone
//! connections created by calling the function `create()` or acquired from a session pool by
//! calling the function `acquireConnection()`. They can be closed by calling the function `close()`
//!  or releasing the last reference to the connection by calling the function `release()`.
//! Connection handles are used to create all handles other than session pools and context handles.
use context::Context;
use context::params::{CommonCreate, ConnCreate};
use dequeue;
use enqueue;
use error::{ErrorKind, Result};
use message::Properties;
use object::Object;
use odpi::{externs, flags};
use odpi::opaque::{ODPIConn, ODPIObject};
use slog::Logger;
use std::{mem, ptr};
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

    /// Begins a distributed transaction using the specified transaction id (XID) made up of the
    /// formatId, transactionId and branchId.
    ///
    /// * `format_id` - the identifier of the format of the XID. A value of -1 indicates that the
    /// entire XID is null.
    /// * `txn_id` - the global transaction id of the XID as a byte string. The maximum length
    /// permitted is 64 bytes.
    /// * `branch_id` - the branch id of the XID as a byte string. The maximum length permitted is
    /// 64 bytes.
    pub fn begin_distrib_trans(&self, format_id: i64, txn_id: &str, branch_id: &str) -> Result<()> {
        let txn_id_s = ODPIStr::from(txn_id);
        let branch_id_s = ODPIStr::from(branch_id);

        if txn_id_s.len() > 64 {
            Err(ErrorKind::TxnId.into())
        } else if branch_id_s.len() > 64 {
            Err(ErrorKind::BranchId.into())
        } else {
            try_dpi!(externs::dpiConn_beginDistribTrans(self.conn,
                                                        format_id,
                                                        txn_id_s.ptr(),
                                                        txn_id_s.len(),
                                                        branch_id_s.ptr(),
                                                        branch_id_s.len()),
                     Ok(()),
                     ErrorKind::Connection("dpiConn_beginDistribTrans".to_string()))
        }
    }

    /// Performs an immediate (asynchronous) termination of any currently executing function on the
    /// server associated with the connection.
    pub fn break_execution(&self) -> Result<()> {
        try_dpi!(externs::dpiConn_breakExecution(self.conn),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_breakExecution".to_string()))
    }

    /// Changes the password of the specified user.
    ///
    /// * `username` - the name of the user whose password is to be changed, as a byte string in the
    /// encoding used for CHAR data.
    /// * `old_password` - the old password of the user whose password is to be changed, as a byte
    /// string in the encoding used for CHAR data.
    /// * `new_password` - the new password of the user whose password is to be changed, as a byte
    /// string in the encoding used for CHAR data.
    pub fn change_password(&self,
                           username: &str,
                           old_password: &str,
                           new_password: &str)
                           -> Result<()> {
        let username_s = ODPIStr::from(username);
        let old_password_s = ODPIStr::from(old_password);
        let new_password_s = ODPIStr::from(new_password);

        try_dpi!(externs::dpiConn_changePassword(self.conn,
                                                 username_s.ptr(),
                                                 username_s.len(),
                                                 old_password_s.ptr(),
                                                 old_password_s.len(),
                                                 new_password_s.ptr(),
                                                 new_password_s.len()),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_changePassword".to_string()))
    }

    /// Closes the connection and makes it unusable for further activity.
    ///
    /// * `mode` - one or more of the values from the enumeration `ODPIConnCloseMode`, OR'ed
    /// together.
    /// * `tag` - a byte string in the encoding used for CHAR data, indicating what tag should be
    /// set on the connection when it is released back to the pool. None is also acceptable when
    /// indicating that the tag should be cleared. This value is ignored unless the close mode
    /// includes the value DPI_MODE_CONN_CLOSE_RETAG.
    pub fn close(&self, mode: flags::ODPIConnCloseMode, tag: Option<&str>) -> Result<()> {
        let tag_s = ODPIStr::from(tag);

        try_dpi!(externs::dpiConn_close(self.conn, mode, tag_s.ptr(), tag_s.len()),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_close".to_string()))
    }

    /// Commits the current active transaction.
    pub fn commit(&self) -> Result<()> {
        try_dpi!(externs::dpiConn_commit(self.conn),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_commit".to_string()))
    }

    /// Creates a standalone connection to a database or acquires a connection from a session pool
    /// and returns a reference to the connection.
    ///
    /// * `context` - the context handle created earlier using the function `Context::create()`. If
    /// the handle is NULL or invalid an error is returned.
    /// * `username` - the name of the user used for authenticating the user, as a byte string in
    /// the encoding used for CHAR data. None is also acceptable if external authentication is being
    /// requested or if credentials were specified when the pool was created.
    /// * `password` - the password to use for authenticating the user, as a byte string in the
    /// encoding used for CHAR data. None is also acceptable if external authentication is being
    /// requested or if credentials were specified when the pool was created.
    /// * `connect_string` - the connect string identifying the database to which a connection is to
    /// be established, as a byte string in the encoding used for CHAR data. None is also acceptable
    /// for local connections (identified by the environment variable $ORACLE_SID) or when a
    /// connection is being acquired from a session pool. This value is ignored when a connection is
    /// being acquired from a session pool.
    /// * `common_create_params` - An optional `CommonCreate` structure which is used to specify
    /// context parameters for connection creation. None is also acceptable in which case all
    /// default parameters will be used when creating the connection. This value is ignored when a
    /// cquiring a connection from a session pool.
    /// * `conn_create_params` - An optional `ConnCreate` structure which is used to specify
    /// parameters for connection creation. None is also acceptable in which case all default
    /// parameters will be used when creating the connection.
    pub fn create(context: &Context,
                  username: Option<&str>,
                  password: Option<&str>,
                  connect_string: Option<&str>,
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

    /// Dequeues a message from a queue.
    ///
    /// * `queue_name` - the name of the queue from which the message is to be dequeued, as a byte
    /// string in the encoding used for CHAR data.
    /// * `options` - a reference to the dequeue options that should be used when dequeuing the
    /// message from the queue.
    /// * `props` -- a reference to the message properties that will be populated with information
    /// from the message that is dequeued.
    pub fn deque_object(&self,
                        queue_name: &str,
                        options: &dequeue::Options,
                        props: &Properties)
                        -> Result<(String, Object)> {
        let queue_s = ODPIStr::from(queue_name);
        let mut payload = unsafe { mem::uninitialized::<ODPIObject>() };
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_deqObject(self.conn,
                                            queue_s.ptr(),
                                            queue_s.len(),
                                            options.inner,
                                            props.inner,
                                            &mut payload,
                                            &mut pdst,
                                            &mut dstlen),
                 Ok((ODPIStr::new(pdst, dstlen).into(), Object { inner: &mut payload })),
                 ErrorKind::Connection("dpiConn_deqObject".to_string()))
    }

    /// Enqueues a message to a queue.
    ///
    /// * `queue_name` - the name of the queue to which the message is to be enqueued, as a byte
    /// string in the encoding used for CHAR data.
    /// * `options` - a reference to the enqueue options that should be used when enqueuing the
    /// message to the queue.
    /// * `props` - a reference to the message properties that will affect the message that is
    /// enqueued.
    pub fn enqueue_object(&self,
                          queue_name: &str,
                          options: &enqueue::Options,
                          props: &Properties)
                          -> Result<(String, Object)> {
        let mut payload = unsafe { mem::uninitialized::<ODPIObject>() };
        let queue_s = ODPIStr::from(queue_name);
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_enqObject(self.conn,
                                            queue_s.ptr(),
                                            queue_s.len(),
                                            options.inner,
                                            props.inner,
                                            &mut payload,
                                            &mut pdst,
                                            &mut dstlen),
                 Ok((ODPIStr::new(pdst, dstlen).into(), Object { inner: &mut payload })),
                 ErrorKind::Connection("dpiConn_enqObject".to_string()))
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
                                     Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
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

    // #[test]
    // fn break_execution() {
    //     let conn = match *CONN {
    //         ConnResult::Ok(ref conn) => conn,
    //         ConnResult::Err(ref _e) => return assert!(false),
    //     };

    //     match conn.break_execution() {
    //         Ok(_) => assert!(true),
    //         Err(_) => assert!(false),
    //     }
    // }

    #[test]
    fn begin_tx_commit() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };

        match conn.begin_distrib_trans(100, "One", "Two") {
            Ok(_) => {
                match conn.commit() {
                    Ok(_) => assert!(true),
                    Err(_) => assert!(false),
                }
            }
            Err(_) => assert!(false),
        }
    }
}
