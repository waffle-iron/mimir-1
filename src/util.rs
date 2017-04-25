//! `oic` utilities
use std::os::raw::c_char;

/// Convert the given string into a (ptr, u32) tuple.
pub fn to_ffi_str(s: &str) -> (*const c_char, u32) {
    let s_ptr = s.as_ptr() as *const c_char;
    #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
    let s_len = s.len() as u32;
    (s_ptr, s_len)
}
