//! Statement handles are used to represent statements of all types (queries, DML, DDL and PL/SQL).
//! They are created by calling the function `dpiConn_prepareStmt()` or the function
//! `dpiSubscr_prepareStmt()`. They are also created implicitly when a variable of type
//! `DPI_ORACLE_TYPE_STMT` is created. Statement handles can be closed by calling the function
//! `dpiStmt_close()` or by releasing the last reference to the statement by calling the function
//! `dpiStmt_release()`.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::flags::{ODPIExecMode, ODPINativeTypeNum};
use odpi::opaque::ODPIStmt;
use odpi::structs::{ODPIData, ODPIQueryInfo};
use public::data::Data;
use public::variable::Var;
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
    pub fn bind_by_name(&self, name: &str, var: Var) -> Result<()> {
        let name_s = ODPIStr::from(name);

        /// TODO: Test this when Var is complete.
        try_dpi!(externs::dpiStmt_bindByName(self.stmt, name_s.ptr(), name_s.len(), var.var()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindByName".to_string()))
    }

    /// Binds a variable to a placeholder in the statement by position. A reference to the variable
    /// is retained by the library and is released when the statement itself is released or a new
    /// variable is bound to the same position.
    pub fn bind_by_pos(&self, pos: u32, var: Var) -> Result<()> {
        try_dpi!(externs::dpiStmt_bindByPos(self.stmt, pos, var.var()),
                 Ok(()),
                 ErrorKind::Statement("dpiStmt_bindByPos".to_string()))
    }

    /// Binds a value to a named placeholder in the statement without the need to create a variable
    /// directly. One is created implicitly and released when the statement is released or a new
    /// value is bound to the same name.
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
    pub fn execute(&self, mode: ODPIExecMode) -> Result<u32> {
        let mut cols_queried = 0;
        try_dpi!(externs::dpiStmt_execute(self.stmt, mode, &mut cols_queried),
                 Ok(cols_queried),
                 ErrorKind::Statement("dpiStmt_execute".to_string()))
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
}

// #[cfg(test)]
// mod test {
//     use {ConnResult, CONN};
//     use odpi::flags::ODPINativeTypeNum::*;
//     use odpi::flags::ODPIOracleTypeNum::*;
//     use odpi::flags;
//     use odpi::structs::{ODPIBytes, ODPIDataValueUnion};
//     use public::data::Data;
//     use util::ODPIStr;

//     #[test]
//     fn add_ref() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select 1 from dual", None, false) {
//             Ok(stmt) => {
//                 match stmt.add_ref() {
//                     Ok(_) => assert!(true),
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn bind_by_name() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.new_var(Varchar, Bytes, 1, 256, false, false) {
//             Ok(var) => {
//                 match conn.prepare_stmt("select * from username where username = :username",
//                                         None,
//                                         false) {
//                     Ok(stmt) => {
//                         match stmt.bind_by_name(":username", var) {
//                             Ok(_) => assert!(true),
//                             Err(_e) => assert!(false),
//                         }
//                     }
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn bind_by_pos() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.new_var(Varchar, Bytes, 1, 256, false, false) {
//             Ok(var) => {
//                 match conn.prepare_stmt("select * from username where username = :username",
//                                         None,
//                                         false) {
//                     Ok(stmt) => {
//                         match stmt.bind_by_pos(1, var) {
//                             Ok(_) => assert!(true),
//                             Err(_e) => assert!(false),
//                         }
//                     }
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn bind_value_by_name() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select * from username where username = :username",
//                                 None,
//                                 false) {
//             Ok(stmt) => {
//                 let blah = ODPIStr::from("test");
//                 let enc = String::from("UTF-8\0");

//                 let odpi_bytes = ODPIBytes {
//                     ptr: blah.ptr() as *mut i8,
//                     length: blah.len(),
//                     encoding: enc.as_ptr() as *const ::std::os::raw::c_char,
//                 };

//                 let data = Data::new(false, ODPIDataValueUnion { as_bytes: odpi_bytes });
//                 match stmt.bind_value_by_name(":username", Bytes, data) {
//                     Ok(_) => assert!(true),
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn bind_value_by_pos() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select * from username where username = :username",
//                                 None,
//                                 false) {
//             Ok(stmt) => {
//                 let blah = ODPIStr::from("test");
//                 let enc = String::from("UTF-8\0");

//                 let odpi_bytes = ODPIBytes {
//                     ptr: blah.ptr() as *mut i8,
//                     length: blah.len(),
//                     encoding: enc.as_ptr() as *const ::std::os::raw::c_char,
//                 };

//                 let data = Data::new(false, ODPIDataValueUnion { as_bytes: odpi_bytes });
//                 match stmt.bind_value_by_pos(1, Bytes, data) {
//                     Ok(_) => assert!(true),
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     // #[test]
//     // fn close() {
//     //     with_stmt!(Ok(stmt) => {
//     //         match stmt.close(None) {
//     //             Ok(_) => assert!(true),
//     //             Err(_e) => assert!(false),
//     //         }
//     //     }; "select 1 from dual")
//     // }

//     #[test]
//     fn execute() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select * from username", None, false) {
//             Ok(stmt) => {
//                 match stmt.execute(flags::EXEC_DEFAULT) {
//                     Ok(cols) => assert!(cols == 2),
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn query_info() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select * from username where username = 'jozias'",
//                                 None,
//                                 false) {
//             Ok(stmt) => {
//                 match stmt.execute(flags::EXEC_DEFAULT) {
//                     Ok(cols) => {
//                         assert!(cols == 2);
//                         match stmt.get_query_info(1) {
//                             Ok(qi) => assert!(qi == "ID"),
//                             Err(_e) => assert!(false),
//                         }
//                         match stmt.get_query_info(2) {
//                             Ok(qi) => assert!(qi == "USERNAME"),
//                             Err(_e) => assert!(false),
//                         }
//                     }
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }

//     #[test]
//     fn query_value() {
//         let conn = match *CONN {
//             ConnResult::Ok(ref conn) => conn,
//             ConnResult::Err(ref _e) => return assert!(false),
//         };
//         match conn.prepare_stmt("select * from username where username = 'jozias'",
//                                 None,
//                                 false) {
//             Ok(stmt) => {
//                 match stmt.execute(flags::EXEC_DEFAULT) {
//                     Ok(cols) => {
//                         assert!(cols == 2);
//                         match stmt.fetch() {
//                             Ok(_) => assert!(true),
//                             Err(_e) => assert!(false),
//                         }
//                         match stmt.get_query_value(1) {
//                             Ok((t, ptr)) => {
//                                 assert!(t == 3003);
//                                 let data = Data::from_ptr(ptr);
//                                 assert!(data.get_double() == 1.0);
//                             }
//                             Err(_e) => assert!(false),
//                         }
//                         match stmt.get_query_value(2) {
//                             Ok((t, ptr)) => {
//                                 assert!(t == 3004);
//                                 let data = Data::from_ptr(ptr);
//                                 assert!(data.get_bytes() == "jozias");
//                             }
//                             Err(_e) => assert!(false),
//                         }
//                     }
//                     Err(_e) => assert!(false),
//                 }
//             }
//             Err(_e) => assert!(false),
//         }
//     }
// }
