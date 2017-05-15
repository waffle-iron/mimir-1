// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Dequeue option handles are used to represent the options specified when dequeuing messages using
//! advanced queueing. They are created by calling the function `Connection::new_deq_ptions()` and
//! are destroyed by releasing the last reference by calling the function
//! `dequeue::Options::release()`.
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIDeqOptions;
use std::ptr;
use util::ODPIStr;

/// Dequeue option handles are used to represent the options specified when dequeuing messages using
/// advanced queueing.
#[derive(Clone)]
pub struct Options {
    /// The ODPI-C DeqOptions pointer.
    inner: *mut ODPIDeqOptions,
}

impl Options {
    /// Get the `inner` value.
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIDeqOptions {
        self.inner
    }

    /// Adds a reference to the dequeue options. This is intended for situations where a reference
    /// to the dequeue options needs to be maintained independently of the reference returned when
    /// the handle was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_addRef(self.inner),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_addRef".to_string()))
    }

    /// Returns the condition that must be satisfied in order for a message to be dequeued. See
    /// function `dequeue::Options::set_condition()` for more information.
    pub fn get_condition(&self) -> Result<String> {
        let mut res = ptr::null();
        let mut res_len = 0;

        try_dpi!(externs::dpiDeqOptions_getCondition(self.inner, &mut res, &mut res_len),
                 {
                     let cond = if res.is_null() {
                         "".to_string()
                     } else {
                         let res_s = ODPIStr::new(res, res_len);
                         res_s.into()
                     };
                     Ok(cond)
                 },
                 ErrorKind::DeqOptions("dpiDeqOptions_getCondition".to_string()))
    }

    /// Returns the name of the consumer that is dequeuing messages. See function
    /// `dequeue::Options::set_consumer_name()` for more information.
    pub fn get_consumer_name(&self) -> Result<String> {
        let mut res = ptr::null();
        let mut res_len = 0;

        try_dpi!(externs::dpiDeqOptions_getConsumerName(self.inner, &mut res, &mut res_len),
                 {
                     let consumer_name = if res.is_null() {
                         "".to_string()
                     } else {
                         let res_s = ODPIStr::new(res, res_len);
                         res_s.into()
                     };
                     Ok(consumer_name)
                 },
                 ErrorKind::DeqOptions("dpiDeqOptions_getConsumerName".to_string()))
    }

    /// Returns the correlation of the message to be dequeued. See function
    /// `dequeue::Options::set_correlation()` for more information.
    pub fn get_correlation(&self) -> Result<String> {
        let mut res = ptr::null();
        let mut res_len = 0;

        try_dpi!(externs::dpiDeqOptions_getCorrelation(self.inner, &mut res, &mut res_len),
                 {
                     let correlation = if res.is_null() {
                         "".to_string()
                     } else {
                         let res_s = ODPIStr::new(res, res_len);
                         res_s.into()
                     };
                     Ok(correlation)
                 },
                 ErrorKind::DeqOptions("dpiDeqOptions_getCorrelation".to_string()))
    }

    /// Returns the mode that is to be used when dequeuing messages.
    pub fn get_mode(&self) -> Result<flags::ODPIDeqMode> {
        let mut deq_mod_ptr = flags::ODPIDeqMode::Remove;

        try_dpi!(externs::dpiDeqOptions_getMode(self.inner, &mut deq_mod_ptr),
                 Ok(deq_mod_ptr),
                 ErrorKind::DeqOptions("dpiDeqOptions_getMode".to_string()))
    }

    /// Returns the identifier of the specific message that is to be dequeued.
    pub fn get_msg_id(&self) -> Result<String> {
        let mut res = ptr::null();
        let mut res_len = 0;

        try_dpi!(externs::dpiDeqOptions_getMsgId(self.inner, &mut res, &mut res_len),
                 {
                     let msg_id = if res.is_null() {
                         "".to_string()
                     } else {
                         let res_s = ODPIStr::new(res, res_len);
                         res_s.into()
                     };
                     Ok(msg_id)
                 },
                 ErrorKind::DeqOptions("dpiDeqOptions_getMsgId".to_string()))
    }

    /// Returns the position of the message that is to be dequeued.
    pub fn get_navigation(&self) -> Result<flags::ODPIDeqNavigation> {
        let mut nav = flags::ODPIDeqNavigation::NextMsg;

        try_dpi!(externs::dpiDeqOptions_getNavigation(self.inner, &mut nav),
                 Ok(nav),
                 ErrorKind::DeqOptions("dpiDeqOptions_getNavigation".to_string()))
    }


    /// Returns the transformation of the message to be dequeued. See function
    /// `dequeue::Options::set_transformation()` for more information.
    pub fn get_transformation(&self) -> Result<String> {
        let mut res = ptr::null();
        let mut res_len = 0;

        try_dpi!(externs::dpiDeqOptions_getTransformation(self.inner, &mut res, &mut res_len),
                 {
                     let transformation = if res.is_null() {
                         "".to_string()
                     } else {
                         let res_s = ODPIStr::new(res, res_len);
                         res_s.into()
                     };
                     Ok(transformation)
                 },
                 ErrorKind::DeqOptions("dpiDeqOptions_getTransformation".to_string()))
    }

    /// Returns whether the message being dequeued is part of the current transaction or constitutes
    /// a transaction on its own.
    pub fn get_visibility(&self) -> Result<flags::ODPIVisibility> {
        let mut visibility = flags::ODPIVisibility::OnCommit;

        try_dpi!(externs::dpiDeqOptions_getVisibility(self.inner, &mut visibility),
                 Ok(visibility),
                 ErrorKind::DeqOptions("dpiDeqOptions_getVisibility".to_string()))
    }

    /// Returns the time to wait, in seconds, for a message matching the search criteria. See
    /// function `dequeue::Options::set_wait()` for more information.
    pub fn get_wait(&self) -> Result<u32> {
        let mut wait = 0;

        try_dpi!(externs::dpiDeqOptions_getWait(self.inner, &mut wait),
                 Ok(wait),
                 ErrorKind::DeqOptions("dpiDeqOptions_getWait".to_string()))
    }

    /// Releases a reference to the dequeue options. A count of the references to the dequeue
    /// options is maintained and when this count reaches zero, the memory associated with the
    /// options is freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_release(self.inner),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_release".to_string()))
    }

    /// Sets the condition which must be true for messages to be dequeued. The condition must be a
    /// valid boolean expression similar to the where clause of a SQL query. The expression can
    /// include conditions on message properties, user data properties and PL/SQL or SQL functions.
    /// User data properties must be prefixed with tab.user_data as a qualifier to indicate the
    /// specific column of the queue table that stores the message payload.
    ///
    /// * `condition` - a string in the encoding used for CHAR data, or None if the condition is to
    /// be cleared.
    pub fn set_condition(&self, condition: Option<&str>) -> Result<()> {
        let cond_s = ODPIStr::from(condition);

        try_dpi!(externs::dpiDeqOptions_setCondition(self.inner, cond_s.ptr(), cond_s.len()),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setCondition".to_string()))
    }

    /// Sets the name of the consumer which will be dequeuing messages. This value should only be
    /// set if the queue is set up for multiple consumers.
    ///
    /// * `consumer_name` - a string in the encoding used for CHAR data, or None if the consumer
    /// name is to be cleared.
    pub fn set_consumer_name(&self, cosumer_name: Option<&str>) -> Result<()> {
        let cons_s = ODPIStr::from(cosumer_name);

        try_dpi!(externs::dpiDeqOptions_setConsumerName(self.inner, cons_s.ptr(), cons_s.len()),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setConsumerName".to_string()))
    }

    /// Sets the correlation of the message to be dequeued. Special pattern matching characters such
    /// as the percent sign (%) and the underscore (_) can be used. If multiple messages satisfy the
    /// pattern, the order of dequeuing is undetermined.
    ///
    /// * `correlation` - a string in the encoding used for CHAR data, or None if the correlation is
    /// to be cleared.
    pub fn set_correlation(&self, correlation: Option<&str>) -> Result<()> {
        let corr_s = ODPIStr::from(correlation);

        try_dpi!(externs::dpiDeqOptions_setCorrelation(self.inner, corr_s.ptr(), corr_s.len()),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setCorrelation".to_string()))
    }

    /// Sets the mode that is to be used when dequeuing messages.
    ///
    /// * `mode` - he mode that should be used. It should be one of the values from the enumeration
    /// `ODPIDeqMode`.
    pub fn set_mode(&self, mode: flags::ODPIDeqMode) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_setMode(self.inner, mode),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setMode".to_string()))
    }

    /// Sets the identifier of the specific message to be dequeued.
    ///
    /// * `msg_id` - a string making up the message identifier, or None if no specific message is to
    /// be dequeued.
    pub fn set_msg_id(&self, msg_id: Option<&str>) -> Result<()> {
        let msg_id_s = ODPIStr::from(msg_id);

        try_dpi!(externs::dpiDeqOptions_setMsgId(self.inner, msg_id_s.ptr(), msg_id_s.len()),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setMsgId".to_string()))
    }

    /// Sets the position in the queue of the message that is to be dequeued.
    ///
    /// * `nav` - the value that should be used. It should be one of the values from the enumeration
    /// `ODPIDeqNavigation`.
    pub fn set_navigation(&self, nav: flags::ODPIDeqNavigation) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_setNavigation(self.inner, nav),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setNavigation".to_string()))
    }

    /// Sets the transformation of the message to be dequeued. The transformation is applied after
    /// the message is dequeued but before it is returned to the application. It must be created
    /// using DBMS_TRANSFORM.
    ///
    /// * `transform` - a string in the encoding used for CHAR data, or None if the transformation
    /// is to be cleared.
    pub fn set_transformation(&self, transform: Option<&str>) -> Result<()> {
        let transform_s = ODPIStr::from(transform);

        try_dpi!(externs::dpiDeqOptions_setTransformation(self.inner,
                                                          transform_s.ptr(),
                                                          transform_s.len()),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setTransformation".to_string()))
    }

    /// Sets whether the message being dequeued is part of the current transaction or constitutes a
    /// transaction on its own.
    ///
    /// * `visibility` - the value that should be used. It should be one of the values from the
    /// enumeration `ODPIVisibility`.
    pub fn set_visibility(&self, visibility: flags::ODPIVisibility) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_setVisibility(self.inner, visibility),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setVisibility".to_string()))
    }

    /// Set the time to wait, in seconds, for a message matching the search criteria.
    ///
    /// * `wait` - the number of seconds to wait for a message matching the search criteria. Any
    /// integer is valid but the predefined constants DPI_DEQ_WAIT_NO_WAIT and DPI_DEQ_WAIT_FOREVER
    /// are provided as a convenience.
    pub fn set_wait(&self, wait: u32) -> Result<()> {
        try_dpi!(externs::dpiDeqOptions_setWait(self.inner, wait),
                 Ok(()),
                 ErrorKind::DeqOptions("dpiDeqOptions_setWait".to_string()))
    }
}

impl From<*mut ODPIDeqOptions> for Options {
    fn from(inner: *mut ODPIDeqOptions) -> Options {
        Options { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags::ODPIConnCloseMode::*;
    use odpi::flags::ODPIDeqMode::*;
    use odpi::flags::ODPIDeqNavigation::*;
    use odpi::flags::ODPIVisibility::*;
    use std::ffi::CString;
    use test::CREDS;

    fn dequeue_opts_res() -> Result<()> {
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

        let dequeue_opts = conn.new_deq_options()?;
        dequeue_opts.add_ref()?;

        dequeue_opts.set_consumer_name(Some("jozias"))?;
        let consumer_name = dequeue_opts.get_consumer_name()?;
        assert_eq!(consumer_name, "jozias");

        dequeue_opts.set_correlation(Some("joz%"))?;
        let correlation = dequeue_opts.get_correlation()?;
        assert_eq!(correlation, "joz%");

        dequeue_opts.set_msg_id(Some("uno"))?;
        // TODO: Fix get_msg_id (causes SIGSEV)
        // let _msg_id = dequeue_opts.get_msg_id()?;
        // assert_eq!(_msg_id, "uno");

        dequeue_opts.set_wait(100000)?;
        let wait = dequeue_opts.get_wait()?;
        assert_eq!(wait, 100000);

        dequeue_opts.set_transformation(Some("tsfm"))?;
        let transformation = dequeue_opts.get_transformation()?;
        assert_eq!(transformation, "tsfm");

        let mut visibility = dequeue_opts.get_visibility()?;
        assert_eq!(visibility, OnCommit);
        dequeue_opts.set_visibility(Immediate)?;
        visibility = dequeue_opts.get_visibility()?;
        assert_eq!(visibility, Immediate);

        let mut mode = dequeue_opts.get_mode()?;
        assert_eq!(mode, Remove);
        dequeue_opts.set_mode(Browse)?;
        mode = dequeue_opts.get_mode()?;
        assert_eq!(mode, Browse);

        let mut nav = dequeue_opts.get_navigation()?;
        assert_eq!(nav, NextMsg);
        dequeue_opts.set_navigation(FirstMsg)?;
        nav = dequeue_opts.get_navigation()?;
        assert_eq!(nav, FirstMsg);

        dequeue_opts.release()?;

        conn.release()?;
        conn.close(DefaultClose, None)?;

        Ok(())
    }

    #[test]
    pub fn dequeue_opts() {
        use std::io::{self, Write};

        match dequeue_opts_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
