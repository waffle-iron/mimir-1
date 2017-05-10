//! `oci` macros
macro_rules! try_dpi {
    ($code:expr, $ret:expr, $err:expr) => {{
        if unsafe { $code } == ::odpi::constants::DPI_SUCCESS {
            $ret
        } else {
            Err($err.into())
        }
    }};
}

#[doc(hidden)]
macro_rules! try_trace(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            trace!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            trace!(log, $($args)+);
        }
    }
);

#[doc(hidden)]
macro_rules! try_debug(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            debug!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            debug!(log, $($args)+);
        }
    }
);

#[doc(hidden)]
macro_rules! try_info(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            info!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            info!(log, $($args)+);
        }
    }
);

#[doc(hidden)]
macro_rules! try_warn(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            warn!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            warn!(log, $($args)+);
        }
    }
);

#[doc(hidden)]
macro_rules! try_error(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            error!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            error!(log, $($args)+);
        }
    }
);

#[doc(hidden)]
macro_rules! try_crit(
    ($l:expr, $($k:expr => $v:expr),+; $($args:tt)+) => {
        if let Some(ref log) = $l {
            crit!(log, $($k => $v),+; $($args)+);
        }
    };
    ($l:expr, $($args:tt)+) => {
        if let Some(ref log) = $l {
            crit!(log, $($args)+);
        }
    }
);

macro_rules! with_conn {
    ($tst:pat => $b:expr) => {{
        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                match Connection::connect(ctxt,
                                          Some(&CREDS[0]),
                                          Some(&CREDS[1]),
                                          "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL") {
                    $tst => { $b }
                    Err(_e) => {
                        use std::io::{self, Write};
                        writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
                            .expect("badness");
                        assert!(false)
                    }
                }
            }
            Err(_e) => assert!(false),
        }
    }}
}

macro_rules! with_stmt {
    ($tst:pat => $b:expr; $stmt:expr) => {{
        match Context::new() {
            Ok(ref mut ctxt) => {
                ctxt.set_encoding("UTF-8");
                ctxt.set_nchar_encoding("UTF-8");
                match Connection::connect(ctxt,
                                          Some(&CREDS[0]),
                                          Some(&CREDS[1]),
                                          "//oic.cbsnae86d3iv.us-east-2.rds.amazonaws.com/ORCL") {
                    Ok(conn) => {
                        match conn.prepare_stmt($stmt, None, false) {
                            $tst => { $b }
                            Err(_e) => {
                                use std::io::{self, Write};
                                writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
                                    .expect("badness");
                                assert!(false)
                            }
                        }
                    }
                    Err(_e) => {
                        use std::io::{self, Write};
                        writeln!(io::stderr(), "{}", error::from_dpi_context(ctxt))
                            .expect("badness");
                        assert!(false)
                    }
                }
            }
            Err(_e) => assert!(false),
        }
    }}
}
