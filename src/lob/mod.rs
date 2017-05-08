//!
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPILob;

///
pub struct Lob {
    ///
    inner: *mut ODPILob,
}

impl Lob {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPILob {
        self.inner
    }

    /// Returns the chunk size of the internal LOB. Reading and writing to the LOB in multiples of
    /// this size will improve performance.
    pub fn get_chunk_size(&self) -> Result<u32> {
        let mut size = 0;
        try_dpi!(externs::dpiLob_getChunkSize(self.inner, &mut size),
                 Ok(size),
                 ErrorKind::Lob("dpiLob_getChunkSize".to_string()))
    }
}

impl From<*mut ODPILob> for Lob {
    fn from(inner: *mut ODPILob) -> Lob {
        Lob { inner: inner }
    }
}
