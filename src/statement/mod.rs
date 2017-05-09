//! Statement handles are used to represent statements of all types (queries, DML, DDL and PL/SQL).
//! They are created by calling the function `dpiConn_prepareStmt()` or the function
//! `dpiSubscr_prepareStmt()`. They are also created implicitly when a variable of type
//! `DPI_ORACLE_TYPE_STMT` is created. Statement handles can be closed by calling the function
//! `dpiStmt_close()` or by releasing the last reference to the statement by calling the function
//! `dpiStmt_release()`.
use data::Data;
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::flags::{ODPIExecMode, ODPINativeTypeNum};
use odpi::opaque::ODPIStmt;
use odpi::structs::{ODPIData, ODPIQueryInfo};
use variable::Var;
use std::{mem, ptr};
use util::ODPIStr;

/// This structure represents statements of all types (queries, DML, DLL and PL/SQL) and is
/// available by handle to a calling application or driver.
pub struct Statement {
    /// The ODPI-C statement
    stmt: *mut ODPIStmt,
}

impl Statement {
    /// Create a new statement from an `ODPIStmt` pointer
    #[doc(hidden)]
    pub fn new(stmt: *mut ODPIStmt) -> Statement {
        Statement { stmt: stmt }
    }

    /// Adds a reference to the statement. This is intended for situations where a reference to the
    /// statement needs to be maintained independently of the reference returned when the statement
    /// was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiStmt_addRef(self.stmt),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_addRef".to_string()))
    }

    /// Binds a variable to a named placeholder in the statement. A reference to the variable is
    /// retained by the library and is released when the statement itself is released or a new
    /// variable is bound to the same name.
    ///
    /// * `name` - a string in the encoding used for CHAR data giving the name of the placeholder
    /// which is to be bound.
    /// * `var` - a variable which is to be bound.
    pub fn bind_by_name(&self, name: &str, var: Var) -> Result<()> {
        let name_s = ODPIStr::from(name);

        /// TODO: Test this when Var is complete.
        try_dpi!(externs::dpiStmt_bindByName(self.stmt, name_s.ptr(), name_s.len(), var.inner()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindByName".to_string()))
    }

    /// Binds a variable to a placeholder in the statement by position. A reference to the variable
    /// is retained by the library and is released when the statement itself is released or a new
    /// variable is bound to the same position.
    ///
    /// * `pos` - the position which is to be bound. The position of a placeholder is determined by
    /// its location in the statement. Placeholders are numbered from left to right, starting from
    /// 1, and duplicate names do not count as additional placeholders.
    /// * `var` - a variable which is to be bound.
    pub fn bind_by_pos(&self, pos: u32, var: Var) -> Result<()> {
        try_dpi!(externs::dpiStmt_bindByPos(self.stmt, pos, var.inner()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindByPos".to_string()))
    }

    /// Binds a value to a named placeholder in the statement without the need to create a variable
    /// directly. One is created implicitly and released when the statement is released or a new
    /// value is bound to the same name.
    ///
    /// * `name` - a string in the encoding used for CHAR data giving the name of the placeholder
    /// which is to be bound.
    /// * `native_type` - the type of data that is being bound. It is expected to be one of the
    /// values from the enumeration `ODPINativeTypeNum`.
    /// * `data` - the data which is to be bound, as a pointer to a `ODPIData` structure. A variable
    /// will be created based on the type of data being bound and a reference to this variable
    /// retained. Once the statement has been executed, this new variable will be released.
    pub fn bind_value_by_name(&self,
                              name: &str,
                              native_type: ODPINativeTypeNum,
                              data: Data)
                              -> Result<()> {
        let name_s = ODPIStr::from(name);

        try_dpi!(externs::dpiStmt_bindValueByName(self.stmt,
                                                  name_s.ptr(),
                                                  name_s.len(),
                                                  native_type,
                                                  data.data()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindValueByName".to_string()))
    }

    /// Binds a value to a placeholder in the statement without the need to create a variable
    /// directly. One is created implicitly and released when the statement is released or a new
    /// value is bound to the same position.
    ///
    /// * `pos` - the position which is to be bound. The position of a placeholder is determined by
    /// its location in the statement. Placeholders are numbered from left to right, starting from
    /// 1, and duplicate names do not count as additional placeholders.
    /// * `native_type` - the type of data that is being bound. It is expected to be one of the
    /// values from the enumeration `ODPINativeTypeNum`.
    /// * `data` - the data which is to be bound, as a pointer to a `ODPIData` structure. A variable
    /// will be created based on the type of data being bound and a reference to this variable
    /// retained. Once the statement has been executed, this new variable will be released.
    pub fn bind_value_by_pos(&self,
                             pos: u32,
                             native_type: ODPINativeTypeNum,
                             data: Data)
                             -> Result<()> {
        try_dpi!(externs::dpiStmt_bindValueByPos(self.stmt, pos, native_type, data.data()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindValueByPos".to_string()))
    }

    /// Closes the statement and makes it unusable for further work immediately, rather than when
    /// the reference count reaches zero.
    ///
    /// * `tag` - a key to associate the statement with in the statement cache, in the encoding used
    /// for CHAR data. None is also acceptable in which case the statement is not tagged. This value
    /// is ignored for statements that are acquired through bind variables (REF CURSOR) or implicit
    /// results.
    pub fn close(&self, tag: Option<&str>) -> Result<()> {
        let tag_s = ODPIStr::from(tag);
        try_dpi!(externs::dpiStmt_close(self.stmt, tag_s.ptr(), tag_s.len()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_close".to_string()))
    }

    // /// Defines the variable that will be used to fetch rows from the statement. A reference to
    //the
    // /// variable will be retained until the next define is performed on the same position or the
    // /// statement is closed.
    // pub fn define(&self, pos: u32, var: &mut Var) -> Result<()> {
    //     Ok(())
    // }

    /// Executes the statement using the bound values. For queries this makes available metadata
    /// which can be acquired using the function dpiStmt_getQueryInfo(). For non-queries, out and
    /// in-out variables are populated with their values.
    ///
    /// * `mode` - one or more of the values from the enumeration `ODPIExecMode`, OR'ed together.
    pub fn execute(&self, mode: ODPIExecMode) -> Result<u32> {
        let mut cols_queried = 0;
        try_dpi!(externs::dpiStmt_execute(self.stmt, mode, &mut cols_queried),
                 Ok(cols_queried),
                 ErrorKind::Statement("dpiStmt_execute".to_string()))
    }

    /// Executes the statement the specified number of times using the bound values. Each bound
    /// variable must have at least this many elements allocated or an error is returned.
    ///
    /// * `mode` - one or more of the values from the enumeration `ODPIExecMode`, OR'ed together.
    /// * `num_iters` - the number of times the statement is executed. Each iteration corresponds to
    /// one of the elements of the array that was bound earlier.
    pub fn execute_many(&self, mode: ODPIExecMode, num_iters: u32) -> Result<()> {
        try_dpi!(externs::dpiStmt_executeMany(self.stmt, mode, num_iters),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_executeMany".to_string()))
    }

    /// Fetches a single row from the statement. If the statement does not refer to a query an error
    /// is returned. All columns that have not been defined prior to this call are implicitly
    /// defined using the metadata made available when the statement was executed.
    pub fn fetch(&self) -> Result<()> {
        let mut found = 0;
        let mut buffer_row_index = 0;

        try_dpi!(externs::dpiStmt_fetch(self.stmt, &mut found, &mut buffer_row_index),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_fetch".to_string()))
    }

    /// Returns information about the column that is being queried.
    pub fn get_query_info(&self, pos: u32) -> Result<String> {
        let mut qi = unsafe { mem::uninitialized::<ODPIQueryInfo>() };

        try_dpi!(externs::dpiStmt_getQueryInfo(self.stmt, pos, &mut qi),
                 {
                     let name_s = ODPIStr::new(qi.name, qi.name_length);
                     Ok(name_s.into())
                 },
                 ErrorKind::Statement("dpiStmt_getQueryInfo".to_string()))
    }

    /// Returns the value of the column at the given position for the currently fetched row, without
    /// needing to provide a variable.
    pub fn get_query_value(&self, pos: u32) -> Result<(i32, *mut ODPIData)> {
        let mut data = ptr::null_mut();
        let mut native_type = 0i32;

        try_dpi!(externs::dpiStmt_getQueryValue(self.stmt, pos, &mut native_type, &mut data),
                 Ok((native_type, data)),
                 ErrorKind::Statement("dpiStmt_getQueryValue".to_string()))
    }


    /// Releases a reference to the statement. A count of the references to the statement is
    /// maintained and when this count reaches zero, the memory associated with the statement is
    /// freed and the statement is closed if that has not already taken place using the function
    /// `close()`.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiStmt_release(self.stmt),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_release".to_string()))
    }
}

#[cfg(test)]
mod test {
    use {ContextResult, CTXT, ENC};
    use connection::Connection;
    use data::Data;
    use error;
    use odpi::flags;
    use odpi::flags::ODPINativeTypeNum::*;
    use odpi::flags::ODPIOracleTypeNum::*;
    use odpi::structs::{ODPIBytes, ODPIData, ODPIDataValueUnion};
    use util::ODPIStr;

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
    fn add_ref_release() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select 1 from dual"), None, false) {
            Ok(stmt) => {
                match stmt.add_ref() {
                    Ok(_) => {
                        match stmt.release() {
                            Ok(_) => assert!(true),
                            Err(_) => assert!(false),
                        }
                    }
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    fn bind_by_name() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.new_var(Varchar, Bytes, 1, 256, false, false) {
            Ok(var) => {
                match conn.prepare_stmt(Some("select * from username where username = :username"),
                                        None,
                                        false) {
                    Ok(stmt) => {
                        match stmt.bind_by_name(":username", var) {
                            Ok(_) => assert!(true),
                            Err(e) => ::error_info(e),
                        }
                    }
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    fn bind_by_pos() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.new_var(Varchar, Bytes, 1, 256, false, false) {
            Ok(var) => {
                match conn.prepare_stmt(Some("select * from username where username = :username"),
                                        None,
                                        false) {
                    Ok(stmt) => {
                        match stmt.bind_by_pos(1, var) {
                            Ok(_) => assert!(true),
                            Err(e) => ::error_info(e),
                        }
                    }
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    fn bind_value_by_name() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select * from username where username = :username"),
                                None,
                                false) {
            Ok(stmt) => {
                let blah = ODPIStr::from("test");
                let enc = String::from("UTF-8\0");

                let odpi_bytes = ODPIBytes {
                    ptr: blah.ptr() as *mut i8,
                    length: blah.len(),
                    encoding: enc.as_ptr() as *const ::std::os::raw::c_char,
                };

                let data = Data::new(false, ODPIDataValueUnion { as_bytes: odpi_bytes });
                match stmt.bind_value_by_name(":username", Bytes, data) {
                    Ok(_) => assert!(true),
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    fn bind_value_by_pos() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select * from username where username = :username"),
                                None,
                                false) {
            Ok(stmt) => {
                let blah = ODPIStr::from("test");
                let enc = String::from("UTF-8\0");

                let odpi_bytes = ODPIBytes {
                    ptr: blah.ptr() as *mut i8,
                    length: blah.len(),
                    encoding: enc.as_ptr() as *const ::std::os::raw::c_char,
                };

                let data = Data::new(false, ODPIDataValueUnion { as_bytes: odpi_bytes });
                match stmt.bind_value_by_pos(1, Bytes, data) {
                    Ok(_) => assert!(true),
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn close() {
        let ctxt = match *CTXT {
            ContextResult::Ok(ref ctxt) => ctxt,
            ContextResult::Err(ref _e) => return assert!(false),
        };
        let ccp = match ctxt.init_common_create_params() {
            Ok(mut ccp) => {
                ccp.set_encoding(ENC.as_ptr());
                ccp.set_nchar_encoding(ENC.as_ptr());
                ccp
            }
            Err(_e) => return assert!(false),
        };
        let conn =
            match Connection::create(ctxt,
                                     Some("jozias"),
                                     Some("chip18jj"),
                                     Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                     Some(ccp),
                                     None) {
                Ok(conn) => conn,
                Err(_e) => return assert!(false),
            };

        match conn.prepare_stmt(Some("select * from username where username = :username"),
                                None,
                                false) {
            Ok(stmt) => {
                match stmt.close(None) {
                    Ok(_) => assert!(true),
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    fn execute() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select * from username"), None, false) {
            Ok(stmt) => {
                match stmt.execute(flags::EXEC_DEFAULT) {
                    Ok(cols) => assert!(cols == 2),
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }
    }

    #[test]
    #[ignore]
    fn execute_many() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        let id_var = match conn.new_var(Number, Int64, 5, 0, false, false) {
            Ok(var) => var,
            Err(e) => return ::error_info(e),
        };

        let mut id_data = match id_var.get_data() {
            Ok(data) => data,
            Err(e) => return ::error_info(e),
        };

        for (idx, data) in id_data.iter_mut().enumerate() {
            let mut d: Data = (data as *mut ODPIData).into();
            d.set_is_null(false);
            d.set_int64(idx as i64);
        }

        let username_var = match conn.new_var(Varchar, Bytes, 5, 2, false, false) {
            Ok(var) => var,
            Err(e) => return ::error_info(e),
        };

        for i in 0..5 {
            match username_var.set_from_bytes(i, "jozias") {
                Ok(_) => assert!(true),
                Err(e) => ::error_info(e),
            }
        }

        match conn.prepare_stmt(Some("insert into username values (:1, :2)"),
                                None,
                                false) {
            Ok(stmt) => {
                match stmt.bind_by_pos(1, id_var) {
                    Ok(_) => assert!(true),
                    Err(e) => ::error_info(e),
                }

                match stmt.bind_by_pos(2, username_var) {
                    Ok(_) => assert!(true),
                    Err(e) => ::error_info(e),
                }

                match stmt.execute_many(flags::EXEC_DEFAULT, 5) {
                    Ok(_) => assert!(true),
                    Err(e) => ::error_info(e),
                }
            }
            Err(e) => ::error_info(e),
        }

    }

    #[test]
    fn query_info() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select * from username where username = 'jozias'"),
                                None,
                                false) {
            Ok(stmt) => {
                match stmt.execute(flags::EXEC_DEFAULT) {
                    Ok(cols) => {
                        assert!(cols == 2);
                        match stmt.get_query_info(1) {
                            Ok(qi) => assert!(qi == "ID"),
                            Err(_e) => assert!(false),
                        }
                        match stmt.get_query_info(2) {
                            Ok(qi) => assert!(qi == "USERNAME"),
                            Err(_e) => assert!(false),
                        }
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }

    #[test]
    fn query_value() {
        let conn = match *CONN {
            ConnResult::Ok(ref conn) => conn,
            ConnResult::Err(ref _e) => return assert!(false),
        };
        match conn.prepare_stmt(Some("select * from username where username = 'jozias'"),
                                None,
                                false) {
            Ok(stmt) => {
                match stmt.execute(flags::EXEC_DEFAULT) {
                    Ok(cols) => {
                        assert!(cols == 2);
                        match stmt.fetch() {
                            Ok(_) => assert!(true),
                            Err(_e) => assert!(false),
                        }
                        match stmt.get_query_value(1) {
                            Ok((t, ptr)) => {
                                assert!(t == 3003);
                                let data: Data = ptr.into();
                                assert!(data.get_double() == 1.0);
                            }
                            Err(_e) => assert!(false),
                        }
                        match stmt.get_query_value(2) {
                            Ok((t, ptr)) => {
                                assert!(t == 3004);
                                let data: Data = ptr.into();
                                assert!(data.get_bytes() == "jozias");
                            }
                            Err(_e) => assert!(false),
                        }
                    }
                    Err(_e) => assert!(false),
                }
            }
            Err(_e) => assert!(false),
        }
    }
}
