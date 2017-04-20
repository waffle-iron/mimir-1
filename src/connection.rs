//! Connection Related Operations
use context::Context;
use error::{self, ErrorKind, Result};
use odpi::constants::DPI_SUCCESS;
use odpi::{externs, flags};
use odpi::opaque::ODPIConn;
use std::ffi::CString;
use std::ptr;

/// ODPI-C dpiConn wrapper.
pub struct Connection {
    #[allow(dead_code)]
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
            Some(username) => (CString::new(username)?.as_ptr(), username.len() as u32),
            None => (ptr::null(), 0),
        };
        let (password_ptr, password_len) = match password {
            Some(password) => (CString::new(password)?.as_ptr(), password.len() as u32),
            None => (ptr::null(), 0),
        };
        let connect_string_ptr = CString::new(connect_string)?.as_ptr();
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
                                              ptr::null(),
                                              ptr::null_mut(),
                                              //   &context.common_create_params(),
                                              //   &mut context.conn_create_params(),
                                              &mut conn);

            if res == DPI_SUCCESS {
                Ok(Connection { conn: conn })
            } else {
                Err(error::from_dpi_context(context).into())
            }
        }
    }

    /// Close the connection.
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
    pub fn close(&self, mode: flags::ODPIConnCloseMode, tag: Option<&str>) -> Result<()> {
        let (tag_ptr, tag_len) = match tag {
            Some(tag) => (CString::new(tag)?.as_ptr(), tag.len() as u32),
            None => (ptr::null(), 0),
        };

        unsafe {
            let res = externs::dpiConn_close(self.conn, mode, tag_ptr, tag_len);

            if res == DPI_SUCCESS {
                Ok(())
            } else {
                Err(ErrorKind::ConnClose.into())
            }
        }
    }
}

#[cfg(test)]
mod test {
    use context::Context;
    use connection::Connection;
    use error;

    #[test]
    fn connect() {
        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                match Connection::connect(ctxt,
                                          Some("bvprod"),
                                          Some("veritiv001"),
                                          "//ecstgdb.int.distco.com:1521/bvnstage") {
                    Ok(conn) => assert!(true),
                    Err(_e) => {
                        println!("{}", error::from_dpi_context(ctxt));
                        assert!(false)
                    }
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
