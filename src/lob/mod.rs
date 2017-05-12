// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
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
