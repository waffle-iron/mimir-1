// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! [NOT IMPL]
//! Message Properties
use chrono::{DateTime, UTC};
use error::{ErrorKind, Result};
use odpi::{externs, flags};
use odpi::opaque::ODPIMsgProps;
use odpi::structs::ODPITimestamp;
use std::ptr;
use util::ODPIStr;

/// ODPI-C Message Props wrapper.
#[derive(Clone)]
pub struct Properties {
    /// The ODPI-C MsgProps pointer.
    inner: *mut ODPIMsgProps,
}

impl Properties {
    #[doc(hidden)]
    pub fn inner(&self) -> *mut ODPIMsgProps {
        self.inner
    }

    /// Adds a reference to the message properties. This is intended for situations where a
    /// reference to the message properties needs to be maintained independently of the reference
    /// returned when the handle was created.
    pub fn add_ref(&self) -> Result<()> {
        try_dpi!(externs::dpiMsgProps_addRef(self.inner),
                 Ok(()),
                 ErrorKind::MsgProps("dpiMsgProps_addRef".to_string()))
    }

    /// Returns the number of attempts that have been made to dequeue a message.
    pub fn get_num_attempts(&self) -> Result<i32> {
        let mut attempts = 0;

        try_dpi!(externs::dpiMsgProps_getNumAttempts(self.inner, &mut attempts),
                 Ok(attempts),
                 ErrorKind::MsgProps("dpiMsgProps_getNumAttempts".to_string()))
    }

    /// Returns the correlation supplied by the producer when the message was enqueued.
    pub fn get_correlation(&self) -> Result<String> {
        let mut corr_ptr = ptr::null();
        let mut corr_len = 0;

        try_dpi!(externs::dpiMsgProps_getCorrelation(self.inner, &mut corr_ptr, &mut corr_len),
                 {
                     let correlation = if corr_ptr.is_null() {
                         "".to_string()
                     } else {
                         let corr_s = ODPIStr::new(corr_ptr, corr_len);
                         corr_s.into()
                     };
                     Ok(correlation)
                 },
                 ErrorKind::Lob("dpiMsgProps_getCorrelation".to_string()))
    }

    /// Returns the number of seconds the enqueued message will be delayed.
    pub fn get_delay(&self) -> Result<i32> {
        let mut delay = 0;
        try_dpi!(externs::dpiMsgProps_getDelay(self.inner, &mut delay),
                 Ok(delay),
                 ErrorKind::MsgProps("dpiMsgProps_getDelay".to_string()))
    }

    /// Returns the mode that was used to deliver the message.
    pub fn get_delivery_mode(&self) -> Result<flags::ODPIMessageDeliveryMode> {
        let mut del_mode_ptr = flags::ODPIMessageDeliveryMode::NotSet;

        try_dpi!(externs::dpiMsgProps_getDeliveryMode(self.inner, &mut del_mode_ptr),
                 Ok(del_mode_ptr.into()),
                 ErrorKind::MsgProps("dpiEnqOptions_getMode".to_string()))
    }

    /// Returns the time that the message was enqueued.
    pub fn get_enq_time(&self) -> Result<DateTime<UTC>> {
        let mut timestamp: ODPITimestamp = Default::default();

        try_dpi!(externs::dpiMsgProps_getEnqTime(self.inner, &mut timestamp),
                 Ok(timestamp.into()),
                 ErrorKind::MsgProps("dpiMsgProps_getEnqTime".to_string()))
    }

    /// Returns the name of the queue to which the message is moved if it cannot be processed
    /// successfully. See function `MsgProps::set_exception_q()` for more information.
    pub fn get_exception_q(&self) -> Result<String> {
        let mut exception_q_ptr = ptr::null();
        let mut exception_q_len = 0;

        try_dpi!(externs::dpiMsgProps_getExceptionQ(self.inner,
                                                    &mut exception_q_ptr,
                                                    &mut exception_q_len),
                 {
                     let exception_q = ODPIStr::new(exception_q_ptr, exception_q_len);
                     Ok(exception_q.into())
                 },
                 ErrorKind::MsgProps("dpiMsgProps_getExceptionQ".to_string()))
    }

    /// Returns the number of seconds the message is available to be dequeued. See function
    /// `MsgProps::set_expiration()` for more information.
    pub fn get_expiration(&self) -> Result<i32> {
        let mut seconds = 0;

        try_dpi!(externs::dpiMsgProps_getExpiration(self.inner, &mut seconds),
                 Ok(seconds),
                 ErrorKind::MsgProps("dpiMsgProps_getExpiration".to_string()))
    }

    /// Returns the id of the message in the last queue that generated this message. See function
    /// `MsgProps::set_original_msg_id()` for more information.
    pub fn get_original_msg_id(&self) -> Result<String> {
        let mut orig_msg_id_ptr = ptr::null();
        let mut orig_msg_id_len = 0;

        try_dpi!(externs::dpiMsgProps_getOriginalMsgId(self.inner,
                                                       &mut orig_msg_id_ptr,
                                                       &mut orig_msg_id_len),
                 {
                     let orig_msg_id = ODPIStr::new(orig_msg_id_ptr, orig_msg_id_len);
                     Ok(orig_msg_id.into())
                 },
                 ErrorKind::MsgProps("dpiMsgProps_getOriginalMsgId".to_string()))

    }

    /// Returns the priority assigned to the message. See function `MsgProps::set_priority()` for
    /// more information.
    pub fn get_priority(&self) -> Result<i32> {
        let mut priority = 0;

        try_dpi!(externs::dpiMsgProps_getPriority(self.inner, &mut priority),
                 Ok(priority),
                 ErrorKind::MsgProps("dpiMsgProps_getPriority".to_string()))
    }

    /// Releases a reference to the message properties. A count of the references to the message
    /// properties is maintained and when this count reaches zero, the memory associated with the
    /// properties is freed.
    pub fn release(&self) -> Result<()> {
        try_dpi!(externs::dpiMsgProps_release(self.inner),
                 Ok(()),
                 ErrorKind::MsgProps("dpiMsgProps_release".to_string()))
    }
}

impl From<*mut ODPIMsgProps> for Properties {
    fn from(inner: *mut ODPIMsgProps) -> Properties {
        Properties { inner: inner }
    }
}

#[cfg(test)]
mod test {
    use chrono::{Datelike, UTC, Timelike};
    use connection::Connection;
    use context::Context;
    use error::Result;
    use odpi::flags::ODPIConnCloseMode::*;
    use odpi::flags::ODPIMessageDeliveryMode::*;
    use std::ffi::CString;
    use test::CREDS;

    fn msg_props_res() -> Result<()> {
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

        let msg_props = conn.new_msg_props()?;
        msg_props.add_ref()?;
        let num_attempts = msg_props.get_num_attempts()?;
        assert_eq!(num_attempts, 0);
        let correlation = msg_props.get_correlation()?;
        assert_eq!(correlation, "");
        let delay = msg_props.get_delay()?;
        assert_eq!(delay, 0);
        let delivery_mode = msg_props.get_delivery_mode()?;
        assert_eq!(delivery_mode, NotSet);
        let enq_time = msg_props.get_enq_time()?;
        let now = UTC::now();
        assert_eq!(enq_time.year(), now.year());
        assert_eq!(enq_time.month(), now.month());
        assert_eq!(enq_time.day(), now.day());
        assert_eq!(enq_time.hour(), now.hour());
        let exception_q = msg_props.get_exception_q()?;
        assert_eq!(exception_q, "");
        let expiration = msg_props.get_expiration()?;
        assert_eq!(expiration, -1);
        let orig_msg_id = msg_props.get_original_msg_id()?;
        assert_eq!(orig_msg_id, "");
        let priority = msg_props.get_priority()?;
        assert_eq!(priority, 0);

        msg_props.release()?;

        conn.release()?;
        conn.close(DefaultClose, None)?;

        Ok(())
    }

    #[test]
    pub fn msg_props() {
        use std::io::{self, Write};

        match msg_props_res() {
            Ok(_) => assert!(true),
            Err(e) => {
                writeln!(io::stderr(), "{}", e).expect("badness");
                assert!(false);
            }
        }
    }
}
