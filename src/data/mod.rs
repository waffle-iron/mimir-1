//! All of these functions are used for getting and setting the various members of the dpiData
//! structure. The members of the structure can be manipulated directly but some languages
//! (such as Go) do not have the ability to manipulate structures containing unions or the ability
//! to process macros. For this reason, none of these functions perform any error checking. They are
//! assumed to be replacements for direct manipulation of the various members of the structure.
use odpi::externs;
use odpi::structs::{ODPIData, ODPIDataValueUnion};
use util::ODPIStr;

/// This structure is used for passing data to and from the database for variables and for
/// manipulating object attributes and collection values.
pub struct Data {
    /// The ODPI-C data pointer.
    data: *mut ODPIData,
}

impl Data {
    /// Create a new `Data` struct;
    #[doc(hidden)]
    pub fn new(is_null: bool, val: ODPIDataValueUnion) -> Data {
        let mut odpi_data = ODPIData {
            is_null: if is_null { 0 } else { 1 },
            value: val,
        };
        Data { data: &mut odpi_data as *mut ODPIData }
    }

    /// Create a `Data` struct from a pointer.
    pub fn from_ptr(data_ptr: *mut ODPIData) -> Data {
        Data { data: data_ptr }
    }

    /// Get the `data` value.
    pub fn data(&self) -> *mut ODPIData {
        self.data
    }

    /// Returns a pointer to the value of the data when the native type is DPI_NATIVE_TYPE_BYTES.
    pub fn get_bytes(&self) -> String {
        unsafe {
            let odpi_bytes_ptr = externs::dpiData_getBytes(self.data);
            let res_s = ODPIStr::new((*odpi_bytes_ptr).ptr, (*odpi_bytes_ptr).length);
            res_s.into()
        }
    }

    /// Returns the value of the data when the native type is DPI_NATIVE_TYPE_DOUBLE.
    pub fn get_double(&self) -> f64 {
        unsafe { externs::dpiData_getDouble(self.data) }
    }
}
