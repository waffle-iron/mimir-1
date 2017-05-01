//! `oic` utilities
use std::os::raw::c_char;
use std::ptr;
use std::slice;

/// Holds a pointer and a length for ODPI-C strings.
#[derive(Clone, Copy, Debug)]
pub struct ODPIStr {
    /// A pointer to the head of the FFI string.
    ptr: *const c_char,
    /// The length of the string.
    len: u32,
}

impl ODPIStr {
    /// Create a new `ODPIStr`.
    pub fn new(ptr: *const c_char, len: u32) -> ODPIStr {
        ODPIStr { ptr: ptr, len: len }
    }

    /// Get the `ptr` value.
    pub fn ptr(&self) -> *const c_char {
        self.ptr
    }

    /// Get the `len` value.
    pub fn len(&self) -> u32 {
        self.len
    }
}

impl Default for ODPIStr {
    fn default() -> ODPIStr {
        ODPIStr {
            ptr: ptr::null(),
            len: 0,
        }
    }
}

impl<'a> From<Option<&'a str>> for ODPIStr {
    fn from(opt_s: Option<&str>) -> ODPIStr {
        match opt_s {
            Some(s) => ODPIStr::from(s),
            None => Default::default(),
        }
    }
}

impl<'a> From<&'a str> for ODPIStr {
    fn from(s: &str) -> ODPIStr {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
        let s_len = s.len() as u32;
        ODPIStr {
            ptr: s.as_ptr() as *const c_char,
            len: s_len,
        }
    }
}

impl From<String> for ODPIStr {
    fn from(s: String) -> ODPIStr {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
        let s_len = s.len() as u32;
        ODPIStr {
            ptr: s.as_ptr() as *const c_char,
            len: s_len,
        }
    }
}

impl From<ODPIStr> for String {
    fn from(s: ODPIStr) -> String {
        let vec = unsafe { slice::from_raw_parts(s.ptr as *mut u8, s.len as usize) };
        String::from_utf8_lossy(vec).into_owned()
    }
}
