//! Connection Related Operations
use context::Context;
use deqopts::DeqOptions;
use enqopts::EnqOptions;
use error::{ErrorKind, Result};
use msgprops::MsgProps;
use object::Object;
use odpi::{externs, flags};
use odpi::opaque::{ODPIConn, ODPIContext};
use odpi::structs::{ODPIEncodingInfo, ODPISubscrCreateParams, ODPIVersionInfo};
use std::mem;
use std::ptr;
use util::{self, ODPIStr};

/// ODPI-C dpiConn wrapper.
pub struct Connection {
    /// The ODPI-C connection.
    conn: *mut ODPIConn,
}

impl Connection {
    /// Create a new connection to a database.
    pub fn connect(context: &Context,
                   username: Option<&str>,
                   password: Option<&str>,
                   connect_string: &str)
                   -> Result<Connection> {
        let username_s = ODPIStr::from(username);
        let password_s = ODPIStr::from(password);
        let connect_string_s = ODPIStr::from(connect_string);
        let mut conn: *mut ODPIConn = ptr::null_mut();

        try_dpi!(externs::dpiConn_create(context.context(),
                                         username_s.ptr(),
                                         username_s.len(),
                                         password_s.ptr(),
                                         password_s.len(),
                                         connect_string_s.ptr(),
                                         connect_string_s.len(),
                                         &context.common_create_params(),
                                         &mut context.conn_create_params(),
                                         &mut conn),
                 Ok(Connection { conn: conn }),
                 ErrorKind::Connection("dpiConn_create".to_string()))
    }

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

    /// Dequeues a message from a queue.
    pub fn deque_object(&self,
                        queue_name: &str,
                        deq_opts: &DeqOptions,
                        msg_props: &MsgProps,
                        obj: &Object)
                        -> Result<String> {
        let queue_s = ODPIStr::from(queue_name);
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_deqObject(self.conn,
                                            queue_s.ptr(),
                                            queue_s.len(),
                                            deq_opts.inner,
                                            msg_props.inner,
                                            obj.inner,
                                            &mut pdst,
                                            &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_deqObject".to_string()))
    }

    /// Enqueues a message to a queue.
    pub fn enqueue_object(&self,
                          queue_name: &str,
                          enq_opts: &EnqOptions,
                          msg_props: &MsgProps,
                          obj: &Object)
                          -> Result<String> {
        let queue_s = util::ODPIStr::from(queue_name);
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_enqObject(self.conn,
                                            queue_s.ptr(),
                                            queue_s.len(),
                                            enq_opts.inner,
                                            msg_props.inner,
                                            obj.inner,
                                            &mut pdst,
                                            &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_enqObject".to_string()))
    }

    /// Get the current schema.
    pub fn current_schema(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_getCurrentSchema(self.conn, &mut pdst, &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_getCurrentSchema".to_string()))
    }

    /// Returns the edition that is being used by the connection.
    pub fn edition(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_getEdition(self.conn, &mut pdst, &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_getEdition".to_string()))
    }

    /// Returns the encoding information used by the connection. This will be equivalent to the
    /// values passed when the standalone connection or session pool was created, or the values
    /// retrieved from the environment variables NLS_LANG and NLS_NCHAR.
    pub fn encoding(&self) -> Result<()> {
        let mut encoding_info = unsafe { mem::uninitialized::<ODPIEncodingInfo>() };
        // TODO: Return the encoding info object.
        try_dpi!(externs::dpiConn_getEncodingInfo(self.conn, &mut encoding_info),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_getEncodingInfo".to_string()))
    }

    /// Returns the external name that is being used by the connection. This value is used when
    /// logging distributed transactions.
    pub fn external_name(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_getExternalName(self.conn, &mut pdst, &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_getEdition".to_string()))
    }

    /// Returns the OCI service context handle in use by the connection.
    pub fn handle(&self) -> Result<ODPIContext> {
        let mut pdst = ptr::null_mut();

        try_dpi!(externs::dpiConn_getHandle(self.conn, &mut pdst),
                 Ok(unsafe { *(pdst as *mut ODPIContext) }),
                 ErrorKind::Connection("dpiConn_getHandle".to_string()))
    }

    /// Returns the internal name that is being used by the connection. This value is used when
    /// logging distributed transactions.
    pub fn internal_name(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_getInternalName(self.conn, &mut pdst, &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_getInternalName".to_string()))
    }

    /// Returns the logical transaction id for the connection. This value is used in Transaction
    /// Guard to determine if the last failed call was completed and if the transaction was
    /// committed using the procedure call dbms_app_cont.get_ltxid_outcome().
    pub fn ltxid(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        try_dpi!(externs::dpiConn_getLTXID(self.conn, &mut pdst, &mut dstlen),
                 Ok(ODPIStr::new(pdst, dstlen).into()),
                 ErrorKind::Connection("dpiConn_getLTXID".to_string()))
    }

    /// Looks up an object type by name in the database and returns a reference to it. The reference
    /// should be released as soon as it is no longer needed.
    pub fn object_type(&self, name: &str) -> Result<()> {
        let mut pobj = ptr::null_mut();
        let name_s = ODPIStr::from(name);
        // TODO: Return object type ref
        try_dpi!(externs::dpiConn_getObjectType(self.conn, name_s.ptr(), name_s.len(), &mut pobj),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_getObjectType".to_string()))
    }

    /// Returns the version information of the Oracle Database to which the connection has been
    /// made.
    pub fn server_version(&self) -> Result<(String, u32, String)> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;
        let mut version_info = unsafe { mem::uninitialized::<ODPIVersionInfo>() };

        try_dpi!(externs::dpiConn_getServerVersion(self.conn,
                                                   &mut pdst,
                                                   &mut dstlen,
                                                   &mut version_info),
                 {
                     let ver_str = format!("{}.{}.{}.{}.{}",
                                           version_info.version_num,
                                           version_info.release_num,
                                           version_info.update_num,
                                           version_info.port_release_num,
                                           version_info.port_update_num);
                     let ver_desc = ODPIStr::new(pdst, dstlen).into();
                     let ver_num = version_info.full_version_num;
                     Ok((ver_str, ver_num, ver_desc))
                 },
                 ErrorKind::Connection("dpiConn_getServerVersion".to_string()))
    }

    /// Returns the size of the statement cache, in number of statements.
    pub fn statement_cache_size(&self) -> Result<u32> {
        let mut size = 0;

        try_dpi!(externs::dpiConn_getStmtCacheSize(self.conn, &mut size),
                 Ok(size),
                 ErrorKind::Connection("dpiConn_getStmtCacheSize".to_string()))
    }

    /// Returns a reference to a new set of dequeue options, used in dequeuing objects from a queue.
    /// The reference should be released as soon as it is no longer needed.
    pub fn new_deque_opts(&self) -> Result<()> {
        let mut deq_ptr = ptr::null_mut();
        // TODO: Return point to deque options.
        try_dpi!(externs::dpiConn_newDeqOptions(self.conn, &mut deq_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newDeqOptions".to_string()))
    }

    /// Returns a reference to a new set of enqueue options, used in enqueuing objects into a queue.
    /// The reference should be released as soon as it is no longer needed.
    pub fn new_enque_opts(&self) -> Result<()> {
        let mut enq_ptr = ptr::null_mut();
        // TODO: Return pointer to enque options.
        try_dpi!(externs::dpiConn_newEnqOptions(self.conn, &mut enq_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newEnqOptions".to_string()))
    }

    /// Returns a reference to a new set of message properties, used in enqueuing and dequeuing
    /// objects in a queue. The reference should be released as soon as it is no longer needed.
    pub fn new_msg_props(&self) -> Result<()> {
        let mut msg_props_ptr = ptr::null_mut();
        // TODO: Return pointer to enque options.
        try_dpi!(externs::dpiConn_newMsgProps(self.conn, &mut msg_props_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newMsgProps".to_string()))
    }

    /// Returns a reference to a subscription which is used for requesting notifications of changes
    /// on tables or queries that are made in the database. The reference should be released as soon
    /// as it is no longer needed.
    pub fn new_subscription(&self) -> Result<()> {
        let mut subscr_ptr = ptr::null_mut();
        let mut subscr_id = 0;
        /// TODO: create acutal params.
        let mut subscr_cp = unsafe { mem::uninitialized::<ODPISubscrCreateParams>() };

        /// TODO: Return pointer and id.
        try_dpi!(externs::dpiConn_newSubscription(self.conn,
                                                  &mut subscr_cp,
                                                  &mut subscr_ptr,
                                                  &mut subscr_id),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newSubscription".to_string()))
    }

    /// Returns a reference to a new temporary LOB which may subsequently be written and bound to a
    /// statement. The reference should be released as soon as it is no longer needed.
    pub fn new_temp_lob(&self, lob_type: flags::ODPIOracleTypeNum) -> Result<()> {
        let mut lob_ptr = ptr::null_mut();

        match lob_type {
            flags::ODPIOracleTypeNum::Clob |
            flags::ODPIOracleTypeNum::NClob |
            flags::ODPIOracleTypeNum::Blob => {}
            _ => return Err(ErrorKind::Connection("invalid oracle type".to_string()).into()),
        }

        /// TODO: Return ptr to temp lob.
        try_dpi!(externs::dpiConn_newTempLob(self.conn, lob_type, &mut lob_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newTempLob".to_string()))
    }

    /// Returns a reference to a new variable which can be used for binding data to a statement or
    /// providing a buffer for querying data from the database. The reference should be released as
    /// soon as it is no longer needed.
    pub fn new_var(&self,
                   oracle_type_num: flags::ODPIOracleTypeNum,
                   native_type_num: flags::ODPINativeTypeNum)
                   -> Result<()> {
        let mut var_ptr = ptr::null_mut();
        let mut data_ptr = ptr::null_mut();
        let object_type = ptr::null_mut();

        /// TODO: This function is incomplete.
        try_dpi!(externs::dpiConn_newVar(self.conn,
                                         oracle_type_num,
                                         native_type_num,
                                         0,
                                         0,
                                         0,
                                         0,
                                         object_type,
                                         &mut var_ptr,
                                         &mut data_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_newVar".to_string()))
    }

    /// Pings the database to verify that the connection is still alive.
    pub fn ping(&self) -> Result<()> {
        try_dpi!(externs::dpiConn_ping(self.conn),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_ping".to_string()))
    }

    /// Prepares a distributed transaction for commit. This function should only be called after
    /// dpiConn_beginDistribTrans() is called and before dpiConn_commit() is called.
    pub fn prepare_distrib_trans(&self) -> Result<()> {
        let mut commit_needed = 0;
        try_dpi!(externs::dpiConn_prepareDistribTrans(self.conn, &mut commit_needed),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_prepareDistribTrans".to_string()))
    }

    /// Returns a reference to a statement prepared for execution. The reference should be released
    /// as soon as it is no longer needed.
    pub fn prepare_stmt(&self, sql: &str, tag: Option<&str>, scrollable: bool) -> Result<()> {
        let sql_s = ODPIStr::from(sql);
        let tag_s = ODPIStr::from(tag);
        let scroll_i = if scrollable { 0 } else { 1 };
        let mut stmt_ptr = ptr::null_mut();

        try_dpi!(externs::dpiConn_prepareStmt(self.conn,
                                              scroll_i,
                                              sql_s.ptr(),
                                              sql_s.len(),
                                              tag_s.ptr(),
                                              tag_s.len(),
                                              &mut stmt_ptr),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_prepareStmt".to_string()))
    }

    /// Sets the current schema to be used on the connection. This has the same effect as the SQL
    /// statement ALTER SESSION SET CURRENT_SCHEMA. The value be changed when the next call
    /// requiring a round trip to the server is performed. If the new schema name does not exist,
    /// the same error is returned as when the alter session statement is executed. The new schema
    /// name is placed before database objects in statement that you execute that do not already
    /// have a schema.
    pub fn set_current_schema(&self, schema: &str) -> Result<()> {
        let curr_schema_s = ODPIStr::from(schema);
        try_dpi!(externs::dpiConn_setCurrentSchema(self.conn,
                                                   curr_schema_s.ptr(),
                                                   curr_schema_s.len()),
                 Ok(()),
                 ErrorKind::Connection("dpiConn_setCurrentSchema".to_string()))
    }
}

#[cfg(test)]
mod test {
    use context::Context;
    use connection::Connection;
    use error;
    use odpi::flags::ODPIConnCloseMode::*;

    macro_rules! with_conn {
        ($tst:pat => $b:expr) => {{
            match Context::new() {
                Ok(ref mut ctxt) => {
                    ctxt.set_encoding("UTF-8");
                    ctxt.set_nchar_encoding("UTF-8");
                    match Connection::connect(ctxt,
                                            Some("jozias"),
                                            Some("chip18jj"),
                                            "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL") {
                        $tst => { $b }
                        Err(_e) => {
                            use std::io::{self, Write};
                            writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
                                .expect("badness");
                            assert!(false)
                        }
                    }
                }
                Err(_e) => assert!(false),
            }
        }}
    }

    #[test]
    fn connect() {
        with_conn!(Ok(conn) => {
            match conn.close(DefaultClose, None) {
                Ok(_) => assert!(true),
                Err(_e) => assert!(false),
            }
        })
    }

    #[test]
    fn add_ref() {
        with_conn!(Ok(conn) => {
            match conn.add_ref() {
                Ok(_) => {
                    let _ = conn.close(DefaultClose, None);
                    assert!(true)
                }
                Err(_e) => assert!(false),
            }
        })
    }

    #[test]
    fn current_schema() {
        with_conn!(Ok(conn) => {
            match conn.set_current_schema("jozias") {
                Ok(_) => {
                    match conn.current_schema() {
                        Ok(schema) => {
                            let _ = conn.close(DefaultClose, None);
                            assert!(schema == "jozias");
                        }
                        Err(_e) => assert!(false),
                    }
                }
                Err(_e) => assert!(false),
            }
        })
    }

    #[test]
    fn edition() {
        with_conn!(Ok(conn) => {
            match conn.edition() {
                Ok(_edition) => {
                    let _ = conn.close(DefaultClose, None);
                    assert!(true)
                }
                Err(_e) => assert!(false),
            }
        })
    }

    #[test]
    fn external_name() {
        with_conn!(Ok(conn) => {
            match conn.external_name() {
                Ok(_external_name) => {
                    let _ = conn.close(DefaultClose, None);
                    assert!(true)
                }
                Err(_e) => assert!(false),
            }
        })
    }

    #[test]
    fn server_version() {
        with_conn!(Ok(conn) => {
            match conn.server_version() {
                Ok((ver, ver_num, ver_desc)) => {
                    assert!(ver == "12.1.0.2.0");
                    assert!(ver_num == 1201000200);
                    assert!(ver_desc ==
                            "Oracle Database 12c Standard Edition Release 12.1.0.2.0 - \
                            64bit Production");
                }
                Err(_e) => assert!(false),
            }
            let _ = conn.close(DefaultClose, None);
         })
    }

    #[test]
    fn statement_cache_size() {
        with_conn!(Ok(conn) => {
            match conn.statement_cache_size() {
                Ok(size) => assert!(size == 20),
                Err(_e) => assert!(false),
            }
            let _ = conn.close(DefaultClose, None);
        })
    }

    #[test]
    fn ping() {
        with_conn!(Ok(conn) => {
            match conn.ping() {
                Ok(_) => assert!(true),
                Err(_e) => assert!(false),
            }
            let _ = conn.close(DefaultClose, None);
        })
    }
}
