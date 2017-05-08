//! Variable handles are used to represent memory areas used for transferring data to and from the
//! database. They are created by calling the function `dpiConn_newVar()`. They are destroyed when
//! the last reference to the variable is released by calling the function `dpiVar_release()`. They
//! are bound to statements by calling the function `dpiStmt_bindByName()` or the function
//! `dpiStmt_bindByPos()`. They can also be used for fetching data from the database by calling the
//! function `dpiStmt_define()`.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPIVar;
use odpi::structs::ODPIData;
use std::{ptr, slice};

// TODO: Implement the setFrom* functions when needed.

/// This structure represents memory areas used for transferring data to and from the database and
/// is available by handle to a calling application or driver.
pub struct Var {
    /// The ODPI-C var
    inner: *mut ODPIVar,
    /// The ODPI-C dpiData array associated with this variable.
    #[allow(dead_code)]
    data_arr: *mut [ODPIData],
}

impl Var {
    /// Create a new `Var` struct from the given parts.
    #[doc(hidden)]
    pub fn new(var: *mut ODPIVar, data_arr_ptr: *mut ODPIData, size: u32) -> Var {
        let da = unsafe { slice::from_raw_parts_mut(data_arr_ptr, size as usize) };
        Var {
            inner: var,
            data_arr: da,
        }
    }

    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIVar {
        self.inner
    }

    /// Adds a reference to the variable. This is intended for situations where a reference to the
    /// variable needs to be maintained independently of the reference returned when the variable
    /// was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiVar_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Var("dpiVar_addRef".to_string()))
    }

    /// Copies the data from one variable to another variable.
    pub fn copy_data(&self, src_pos: u32, dst: &mut Var, dst_pos: u32) -> Result<()> {
        try_dpi!(externs::dpiVar_copyData(dst.inner(), dst_pos, self.inner, src_pos),
                 Ok(()),
                 ErrorKind::Var("dpiVar_copyData".to_string()))
    }

    /// Returns a pointer to an array of `ODPIData` structures used for transferring data to and
    /// from the database. These structures are allocated by the variable itself and are made
    /// available when the variable is first created using the function `dpiConn_newVar()`. If a DML
    /// returning statement is executed, however, the number of allocated elements can change in
    /// addition to the memory location.
    pub fn get_data(&self) -> Result<u32> {
        let mut num_elements = 0;
        let mut data_arr_ptr = ptr::null_mut();

        try_dpi!(externs::dpiVar_getData(self.inner, &mut num_elements, &mut data_arr_ptr),
                 Ok(num_elements),
                 ErrorKind::Var("dpiVar_getData".to_string()))
    }

    /// Returns the number of elements in a PL/SQL index-by table if the variable was created as an
    /// array by the function `dpiConn_newVar()`. If the variable is one of the output bind
    /// variables of a DML returning statement, however, the value returned will correspond to the
    /// number of rows returned by the DML returning statement. In all other cases, the value
    /// returned will be the number of elements the variable was created with.
    pub fn get_num_elements(&self) -> Result<u32> {
        let mut num_elements = 0;
        try_dpi!(externs::dpiVar_getNumElementsInArray(self.inner, &mut num_elements),
                 Ok(num_elements),
                 ErrorKind::Var("dpiVar_getNumElementsInArray".to_string()))
    }

    /// Returns the size of the buffer used for one element of the array used for fetching/binding
    /// Oracle data.
    pub fn get_size_in_bytes(&self) -> Result<u32> {
        let mut size = 0;
        try_dpi!(externs::dpiVar_getSizeInBytes(self.inner, &mut size),
                 Ok(size),
                 ErrorKind::Var("dpiVar_getSizeInBytes".to_string()))
    }

    /// Releases a reference to the variable. A count of the references to the variable is
    /// maintained and when this count reaches zero, the memory associated with the variable is
    /// freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiVar_release(self.inner),
                 Ok(()),
                 ErrorKind::Var("dpiVar_release".to_string()))
    }
}
