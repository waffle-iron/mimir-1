//! This structure is used for transferring encoding information from ODPI-C.
use odpi::structs::ODPIEncodingInfo;
use std::ffi::CStr;

/// This structure is used for transferring encoding information from ODPI-C.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Info {
    /// The encoding used for CHAR data
    encoding: String,
    /// The encoding used for NCHAR data,
    nchar_encoding: String,
    /// The maximum number of bytes required for each character in the encoding used for CHAR data.
    /// This value is used when calculating the size of buffers required when lengths in characters
    /// are provided.
    max_bytes_per_char: i32,
    /// The maximum number of bytes required for each character in the encoding used for NCHAR data.
    /// Since this information is not directly available from Oracle it is only accurate if the
    /// encodings used for CHAR and NCHAR data are identical or one of ASCII or UTF-8; otherwise a
    /// value of 4 is assumed. This value is used when calculating the size of buffers required when
    /// lengths in characters are provided.
    max_bytes_per_nchar: i32,
}

impl Info {
    /// Get the `encoding` value.
    pub fn encoding(&self) -> &str {
        &self.encoding
    }

    /// Get the `nchar_encoding` value.
    pub fn nchar_encoding(&self) -> &str {
        &self.nchar_encoding
    }

    /// Get the `max_bytes_per_char` value.
    pub fn max_bytes_per_char(&self) -> i32 {
        self.max_bytes_per_char
    }

    /// Get the `max_bytes_per_nchar` value.
    pub fn max_bytes_per_nchar(&self) -> i32 {
        self.max_bytes_per_nchar
    }
}

impl From<ODPIEncodingInfo> for Info {
    fn from(oei: ODPIEncodingInfo) -> Info {
        unsafe {
            let enc = CStr::from_ptr(oei.encoding);
            let nchar_enc = CStr::from_ptr(oei.nchar_encoding);

            Info {
                encoding: enc.to_string_lossy().into_owned(),
                nchar_encoding: nchar_enc.to_string_lossy().into_owned(),
                max_bytes_per_char: oei.max_bytes_per_character,
                max_bytes_per_nchar: oei.nchar_max_bytes_per_character,
            }
        }
    }
}
