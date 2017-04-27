//! Connection Related Operations
use context::Context;
use deqopts::DeqOptions;
use enqopts::EnqOptions;
use error::{ErrorKind, Result};
use msgprops::MsgProps;
use object::Object;
use odpi::constants::DPI_SUCCESS;
use odpi::{externs, flags};
use odpi::opaque::ODPIConn;
use odpi::structs::{ODPIEncodingInfo, ODPIVersionInfo};
use std::mem;
use std::os::raw::c_long;
use std::ptr;
use util::{self, ODPIStr};

/// ODPI-C dpiConn wrapper.
pub struct Connection {
    /// The ODPI-C connection.
    conn: *mut ODPIConn,
}

impl Connection {
    /// Create a new connection to a database.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
    pub fn connect(context: &Context,
                   username: Option<&str>,
                   password: Option<&str>,
                   connect_string: &str)
                   -> Result<Connection> {
        let username_s = ODPIStr::from(username);
        let password_s = ODPIStr::from(password);
        let connect_string_s = ODPIStr::from(connect_string);
        let mut conn: *mut ODPIConn = ptr::null_mut();

        unsafe {
            let res = externs::dpiConn_create(context.context(),
                                              username_s.ptr(),
                                              username_s.len(),
                                              password_s.ptr(),
                                              password_s.len(),
                                              connect_string_s.ptr(),
                                              connect_string_s.len(),
                                              &context.common_create_params(),
                                              &mut context.conn_create_params(),
                                              &mut conn);

            if res == DPI_SUCCESS {
                Ok(Connection { conn: conn })
            } else {
                Err(ErrorKind::Connection("dpiConn_create".to_string()).into())
            }
        }
    }

    /// Adds a reference to the connection. This is intended for situations where a reference to the
    /// connection needs to be maintained independently of the reference returned when the
    /// connection was created.
    pub fn add_ref(&self) -> Result<()> {
        unsafe {
            let res = externs::dpiConn_addRef(self.conn);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_addRef".to_string()).into())
            }
        }
    }

    /// Begins a distributed transaction using the specified transaction id (XID) made up of the
    /// formatId, transactionId and branchId.
    pub fn begin_distrib_trans(&self,
                               format_id: isize,
                               txn_id: &str,
                               branch_id: &str)
                               -> Result<()> {
        let txn_id_s = ODPIStr::from(txn_id);
        let branch_id_s = ODPIStr::from(branch_id);

        if txn_id_s.len() > 64 {
            Err(ErrorKind::TxnId.into())
        } else if branch_id_s.len() > 64 {
            Err(ErrorKind::BranchId.into())
        } else {
            unsafe {
                let res = externs::dpiConn_beginDistribTrans(self.conn,
                                                             format_id as c_long,
                                                             txn_id_s.ptr(),
                                                             txn_id_s.len(),
                                                             branch_id_s.ptr(),
                                                             branch_id_s.len());

                if res == DPI_SUCCESS {
                    Ok(())
                } else {
                    Err(ErrorKind::Connection("dpiConn_beginDistribTrans".to_string()).into())
                }
            }
        }
    }

    /// Performs an immediate (asynchronous) termination of any currently executing function on the
    /// server associated with the connection.
    pub fn break_execution(&self) -> Result<()> {
        unsafe {
            let res = externs::dpiConn_breakExecution(self.conn);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_breakExecution".to_string()).into())
            }
        }
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

        unsafe {
            let res = externs::dpiConn_changePassword(self.conn,
                                                      username_s.ptr(),
                                                      username_s.len(),
                                                      old_password_s.ptr(),
                                                      old_password_s.len(),
                                                      new_password_s.ptr(),
                                                      new_password_s.len());

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_changePassword".to_string()).into())
            }
        }
    }

    /// Closes the connection and makes it unusable for further activity.
    pub fn close(&self, mode: flags::ODPIConnCloseMode, tag: Option<&str>) -> Result<()> {
        let tag_s = ODPIStr::from(tag);

        unsafe {
            let res = externs::dpiConn_close(self.conn, mode, tag_s.ptr(), tag_s.len());

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_close".to_string()).into())
            }
        }
    }

    /// Commits the current active transaction.
    pub fn commit(&self) -> Result<()> {
        unsafe {
            let res = externs::dpiConn_commit(self.conn);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_commit".to_string()).into())
            }
        }
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

        unsafe {
            let res = externs::dpiConn_deqObject(self.conn,
                                                 queue_s.ptr(),
                                                 queue_s.len(),
                                                 deq_opts.inner,
                                                 msg_props.inner,
                                                 obj.inner,
                                                 &mut pdst,
                                                 &mut dstlen);
            if res == DPI_SUCCESS {
                Ok(ODPIStr::new(pdst, dstlen).into())
            } else {
                Err(ErrorKind::Connection("dpiConn_deqObject".to_string()).into())
            }
        }
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

        unsafe {
            let res = externs::dpiConn_enqObject(self.conn,
                                                 queue_s.ptr(),
                                                 queue_s.len(),
                                                 enq_opts.inner,
                                                 msg_props.inner,
                                                 obj.inner,
                                                 &mut pdst,
                                                 &mut dstlen);
            if res == DPI_SUCCESS {
                Ok(ODPIStr::new(pdst, dstlen).into())
            } else {
                Err(ErrorKind::Connection("dpiConn_enqObject".to_string()).into())
            }
        }
    }

    /// Get the current schema.
    pub fn current_schema(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        unsafe {
            let res = externs::dpiConn_getCurrentSchema(self.conn, &mut pdst, &mut dstlen);

            if res == DPI_SUCCESS {
                Ok(ODPIStr::new(pdst, dstlen).into())
            } else {
                Err(ErrorKind::Connection("dpiConn_getCurrentSchema".to_string()).into())
            }
        }
    }

    /// Returns the edition that is being used by the connection.
    pub fn edition(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        unsafe {
            let res = externs::dpiConn_getEdition(self.conn, &mut pdst, &mut dstlen);

            if res == DPI_SUCCESS {
                Ok(ODPIStr::new(pdst, dstlen).into())
            } else {
                Err(ErrorKind::Connection("dpiConn_getEdition".to_string()).into())
            }
        }
    }

    /// Returns the encoding information used by the connection. This will be equivalent to the 
    /// values passed when the standalone connection or session pool was created, or the values 
    /// retrieved from the environment variables NLS_LANG and NLS_NCHAR.
    pub fn encoding(&self) -> Result<()> {
        unsafe {
            let mut encoding_info = mem::uninitialized::<ODPIEncodingInfo>();
            let res = externs::dpiConn_getEncodingInfo(self.conn, &mut encoding_info);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::Connection("dpiConn_getEncodingInfo".to_string()).into())
            }
        }
    }

    /// Returns the external name that is being used by the connection. This value is used when 
    /// logging distributed transactions.
    pub fn external_name(&self) -> Result<String> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;

        unsafe {
            let res = externs::dpiConn_getExternalName(self.conn, &mut pdst, &mut dstlen);

            if res == DPI_SUCCESS {
                Ok(ODPIStr::new(pdst, dstlen).into())
            } else {
                Err(ErrorKind::Connection("dpiConn_getEdition".to_string()).into())
            }
        }
    }

    /// Returns the version information of the Oracle Database to which the connection has been
    /// made.
    pub fn server_version(&self) -> Result<(String, u32, String)> {
        let mut pdst = ptr::null();
        let mut dstlen = 0;
        unsafe {
            let mut version_info = mem::uninitialized::<ODPIVersionInfo>();
            let res = externs::dpiConn_getServerVersion(self.conn,
                                                        &mut pdst,
                                                        &mut dstlen,
                                                        &mut version_info);

            if res == DPI_SUCCESS {
                let ver_str = format!("{}.{}.{}.{}.{}",
                                      version_info.version_num,
                                      version_info.release_num,
                                      version_info.update_num,
                                      version_info.port_release_num,
                                      version_info.port_update_num);
                let ver_desc = ODPIStr::new(pdst, dstlen).into();
                let ver_num = version_info.full_version_num;
                Ok((ver_str, ver_num, ver_desc))
            } else {
                Err(ErrorKind::Connection("dpiConn_getServerVersion".to_string()).into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use context::Context;
    use connection::Connection;
    use error;
    use odpi::flags::ODPIConnCloseMode::*;
    //
    // fn get_context() -> Result<Context> {
    //     match Context::new() {
    //         Ok(mut ctxt) => {
    //             ctxt.set_encoding("UTF-8");
    //             ctxt.set_nchar_encoding("UTF-8");
    //             Ok(ctxt)
    //         }
    //         Err(e) => Err(e),
    //     }
    // }
    //
    // fn get_connection(ctxt: &Context) -> Result<Connection> {
    //     Connection::connect(ctxt,
    //                         Some("bvprod"),
    //                         Some("veritiv001"),
    //                         "//ecstgdb.int.distco.com/bvnstage")
    // }

    #[test]
    fn connect() {
        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                match Connection::connect(ctxt,
                                          Some("jozias"),
                                          Some("chip18jj"),
                                          "//localhost/xe") {
                    Ok(conn) => {
                        match conn.close(DefaultClose, None) {
                            Ok(_) => assert!(true),
                            Err(_e) => {
                                println!("{}", error::from_dpi_context(ctxt));
                                assert!(false)
                            }
                        }
                        assert!(true)
                    }
                    Err(_e) => {
                        println!("{}", error::from_dpi_context(ctxt));
                        assert!(false)
                    }
                }
            }
            Err(_e) => assert!(false),
        }
    }

    // #[test]
    // fn add_ref() {
    //     match get_context().and_then(|ctxt| get_connection(&ctxt)) {
    //         Ok(conn) => {
    //             match conn.add_ref() {
    //                 Ok(_) => {
    //                     let _ = conn.close(DefaultClose, None);
    //                     assert!(true)
    //                 }
    //                 Err(_e) => assert!(false),
    //             }
    //         }
    //         Err(_e) => assert!(false),
    //     }
    // }
    //
    // #[test]
    // fn current_schema() {
    //     use std::io::{self, Write};
    //
    //     match Context::new() {
    //         Ok(ref mut ctxt) => {
    //             ctxt.set_encoding("UTF-8");
    //             ctxt.set_nchar_encoding("UTF-8");
    //             match Connection::connect(ctxt,
    //                                       Some("bvprod"),
    //                                       Some("veritiv001"),
    //                                       "//ecstgdb.int.distco.com/bvnstage") {
    //                 Ok(conn) => {
    //                     match conn.current_schema() {
    //                         Ok(_schema) => {
    //                             let _ = conn.close(DefaultClose, None);
    //                             assert!(true)
    //                         }
    //                         Err(_e) => {
    //                             writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
    //                                 .expect("badness");
    //                             assert!(false)
    //                         }
    //                     }
    //                 }
    //                 Err(_e) => {
    //                     writeln!(io::stderr(),
    //                     "{}", error::from_dpi_context(ctxt)).expect("badness");
    //                     assert!(false)
    //                 }
    //             }
    //         }
    //         Err(_e) => assert!(false),
    //     }
    // }
    //
    // #[test]
    // fn edition() {
    //     use std::io::{self, Write};
    //
    //     match Context::new() {
    //         Ok(ref mut ctxt) => {
    //             ctxt.set_encoding("UTF-8");
    //             ctxt.set_nchar_encoding("UTF-8");
    //             match Connection::connect(ctxt,
    //                                       Some("bvprod"),
    //                                       Some("veritiv001"),
    //                                       "//ecstgdb.int.distco.com/bvnstage") {
    //                 Ok(conn) => {
    //                     match conn.edition() {
    //                         Ok(_edition) => {
    //                             let _ = conn.close(DefaultClose, None);
    //                             assert!(true)
    //                         }
    //                         Err(_e) => {
    //                             writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
    //                                 .expect("badness");
    //                             assert!(false)
    //                         }
    //                     }
    //                 }
    //                 Err(_e) => {
    //                     writeln!(io::stderr(),
    //                     "{}", error::from_dpi_context(ctxt)).expect("badness");
    //                     assert!(false)
    //                 }
    //             }
    //         }
    //         Err(_e) => assert!(false),
    //     }
    // }

    #[test]
    fn server_version() {
        use std::io::{self, Write};

        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                match Connection::connect(ctxt,
                                          Some("jozias"),
                                          Some("chip18jj"),
                                          "//localhost/xe") {
                    Ok(conn) => {
                        match conn.server_version() {
                            Ok((ver, ver_num, ver_desc)) => {
                                assert!(ver == "11.2.0.2.0");
                                assert!(ver_num == 1102000200);
                                assert!(ver_desc ==
                                        "Oracle Database 11g Express Edition Release \
                                11.2.0.2.0 - 64bit Production");
                            }
                            Err(_e) => {
                                writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
                                    .expect("badness");
                                assert!(false)
                            }
                        }
                        let _ = conn.close(DefaultClose, None);
                    }
                    Err(_e) => {
                        writeln!(io::stderr(),
                        "{}", error::from_dpi_context(ctxt)).expect("badness");
                        assert!(false)
                    }
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
