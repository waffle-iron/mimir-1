// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! LOB handles are used to represent large objects (CLOB, BLOB, NCLOB, BFILE). Both persistent and
//! temporary large objects can be represented. LOB handles can be created by calling the function
//! `Connection::new_temp_lob()` or are created implicitly when a variable of type
//! DPI_ORACLE_TYPE_CLOB, DPI_ORACLE_TYPE_NCLOB, DPI_ORACLE_TYPE_BLOB or DPI_ORACLE_TYPE_BFILE is
//! created and are destroyed when the last reference is released by calling the function
//! `Lob::release()`. They are used for reading and writing data to the database in smaller pieces
//! than is contained in the large object.
use error::{ErrorKind, Result};
use odpi::externs;
use odpi::opaque::ODPILob;

/// LOB handles are used to represent large objects (CLOB, BLOB, NCLOB, BFILE).
pub struct Lob {
    /// The ODPI-C LOB pointer.
    inner: *mut ODPILob,
}

impl Lob {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPILob {
        self.inner
    }

    /// Adds a reference to the LOB. This is intended for situations where a reference to the LOB
    /// needs to be maintained independently of the reference returned when the LOB was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiLob_addRef(self.inner),
                 Ok(()),
                 ErrorKind::Lob("dpiLob_addRef".to_string()))
    }

    /// Closes the LOB resource. This should be done when a batch of writes has been completed so
    /// that the indexes associated with the LOB can be updated. It should only be performed if a
    /// call to function `Lob::open_resource()` has been performed.
    pub fn close_resource(&self) -> Result<()> {
        try_dpi!(externs::dpiLob_closeResource(self.inner),
                 Ok(()),
                 ErrorKind::Lob("dpiLob_closeResource".to_string()))
    }

    /// Creates an independent copy of a LOB and returns a reference to the newly created LOB. This
    /// reference should be released as soon as it is no longer needed.
    pub fn copy(&self, dst: &mut Lob) -> Result<()> {
        try_dpi!(externs::dpiLob_copy(self.inner, dst),
                 Ok((), ErrorKind::Lob("dpiLob_copy".to_string()))
    }

    /// Returns the chunk size of the internal LOB. Reading and writing to the LOB in multiples of
    /// this size will improve performance.
    pub fn get_chunk_size(&self) -> Result<u32> {
        let mut size = 0;
        try_dpi!(externs::dpiLob_getChunkSize(self.inner, &mut size),
                 Ok(size),
                 ErrorKind::Lob("dpiLob_getChunkSize".to_string()))
    }

    /// Opens the LOB resource for writing. This will improve performance when writing to the LOB in
    /// chunks and there are functional or extensible indexes associated with the LOB. If this
    /// function is not called, the LOB resource will be opened and closed for each write that is
    /// performed. A call to the function `Lob::close_resource()` should be done before performing a
    /// call to the function `Connection::commit()`.
    pub fn open_resource(&self) -> Result<()> {
        try_dpi!(externs::dpiLob_openResource(self.inner),
                 Ok(()),
                 ErrorKind::Lob("dpiLob_openResource".to_string()))
    }

    /// Releases a reference to the LOB. A count of the references to the LOB is maintained and when
    /// this count reaches zero, the memory associated with the LOB is freed. The LOB is also closed
    /// unless that has already taken place using the function `Lob:::close()`.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiLob_release(self.inner),
                 Ok(()),
                 ErrorKind::Lob("dpiLob_release".to_string()))
    }
}

impl From<*mut ODPILob> for Lob {
    fn from(inner: *mut ODPILob) -> Lob {
        Lob { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags::ODPIConnCloseMode::*;
    use odpi::flags::ODPIOracleTypeNum::*;
    use std::ffi::CString;
    use test::CREDS;

    fn lob_res() -> Result<()> {
        let ctxt = Context::create()?;

        let mut ccp = ctxt.init_common_create_params()?;
        let enc_cstr = CString::new("UTF-8").expect("badness");
        ccp.set_encoding(enc_cstr.as_ptr());
        ccp.set_nchar_encoding(enc_cstr.as_ptr());

        let conn = Connection::create(&ctxt,
                                      Some(&CREDS[0]),
                                      Some(&CREDS[1]),
                                      Some("//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL"),
                                      Some(ccp),
                                      None)?;

        conn.add_ref()?;

        let temp_lob = conn.new_temp_lob(Clob)?;
        temp_lob.add_ref()?;

        temp_lob.release()?;

        conn.release()?;
        conn.close(DefaultClose, None)?;

        Ok(())
    }

    #[test]
    pub fn lob() {
        use std::io::{self, Write};

        match lob_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
