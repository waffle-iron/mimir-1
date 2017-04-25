//! Connection Related Operations
use context::Context;
use error::{self, ErrorKind, Result};
use odpi::constants::DPI_SUCCESS;
use odpi::{externs, flags};
use odpi::opaque::ODPIConn;
use std::os::raw::{c_char, c_long};
use std::ptr;
use util;

/// ODPI-C dpiConn wrapper.
pub struct Connection {
    /// The context associated with this connection.
    context: Context,
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
        let (username_ptr, username_len) = match username {
            Some(username) => (username.as_ptr() as *const c_char, username.len() as u32),
            None => (ptr::null(), 0),
        };
        let (password_ptr, password_len) = match password {
            Some(password) => (password.as_ptr() as *const c_char, password.len() as u32),
            None => (ptr::null(), 0),
        };
        let connect_string_ptr = connect_string.as_ptr() as *const c_char;
        let connect_string_len = connect_string.len() as u32;
        let mut conn: *mut ODPIConn = ptr::null_mut();

        unsafe {
            let res = externs::dpiConn_create(context.context(),
                                              username_ptr,
                                              username_len,
                                              password_ptr,
                                              password_len,
                                              connect_string_ptr,
                                              connect_string_len,
                                              &context.common_create_params(),
                                              &mut context.conn_create_params(),
                                              &mut conn);

            if res == DPI_SUCCESS {
                Ok(Connection {
                       context: context.clone(),
                       conn: conn,
                   })
            } else {
                Err(error::from_dpi_context(context).into())
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
                Err(error::from_dpi_context(&self.context).into())
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
        let txn_id_ptr = txn_id.as_ptr() as *const c_char;
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
        let txn_id_len = txn_id.len() as u32;
        let branch_id_ptr = branch_id.as_ptr() as *const c_char;
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
        let branch_id_len = branch_id.len() as u32;

        if txn_id_len > 64 {
            Err(ErrorKind::TxnId.into())
        } else if branch_id_len > 64 {
            Err(ErrorKind::BranchId.into())
        } else {
            unsafe {
                let res = externs::dpiConn_beginDistribTrans(self.conn,
                                                             format_id as c_long,
                                                             txn_id_ptr,
                                                             txn_id_len,
                                                             branch_id_ptr,
                                                             branch_id_len);

                if res == DPI_SUCCESS {
                    Ok(())
                } else {
                    Err(error::from_dpi_context(&self.context).into())
                }
            }
        }
    }

    /// Changes the password of the specified user.
    pub fn change_password(&self,
                           username: &str,
                           old_password: &str,
                           new_password: &str)
                           -> Result<()> {
        let (username_ptr, username_len) = util::to_ffi_str(username);
        let (old_password_ptr, old_password_len) = util::to_ffi_str(old_password);
        let (new_password_ptr, new_password_len) = util::to_ffi_str(new_password);

        unsafe {
            let res = externs::dpiConn_changePassword(self.conn,
                                                      username_ptr,
                                                      username_len,
                                                      old_password_ptr,
                                                      old_password_len,
                                                      new_password_ptr,
                                                      new_password_len);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(error::from_dpi_context(&self.context).into())
            }
        }
    }

    /// Close the connection.
    pub fn close(&self, mode: flags::ODPIConnCloseMode, tag: Option<&str>) -> Result<()> {
        let (tag_ptr, tag_len) = match tag {
            Some(tag) => {
                #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
                let len = tag.len() as u32;
                (tag.as_ptr() as *const c_char, len)
            }
            None => (ptr::null(), 0),
        };

        unsafe {
            let res = externs::dpiConn_close(self.conn, mode, tag_ptr, tag_len);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(error::from_dpi_context(&self.context).into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use context::Context;
    use connection::Connection;
    use error::{self, Result};
    use odpi::flags::ODPIConnCloseMode::*;

    fn get_context() -> Result<Context> {
        match Context::new() {
            Ok(mut ctxt) => {
                ctxt.set_encoding("UTF-8\0");
                ctxt.set_nchar_encoding("UTF-8\0");
                Ok(ctxt)
            }
            Err(e) => Err(e),
        }
    }

    fn get_connection(ctxt: &Context) -> Result<Connection> {
        Connection::connect(ctxt,
                            Some("bvprod"),
                            Some("veritiv001"),
                            "//ecstgdb.int.distco.com/bvnstage")
    }

    #[test]
    fn connect() {
        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8\0");
                ctxt.set_nchar_encoding("UTF-8\0");
                match Connection::connect(ctxt,
                                          Some("bvprod"),
                                          Some("veritiv001"),
                                          "//ecstgdb.int.distco.com/bvnstage") {
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

    #[test]
    fn add_ref() {
        match get_context().and_then(|ctxt| get_connection(&ctxt)) {
            Ok(conn) => {
                match conn.add_ref() {
                    Ok(_) => {
                        let _ = conn.close(DefaultClose, None);
                        assert!(true)
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
