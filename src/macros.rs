//! `oci` macros
macro_rules! try_dpi {
    ($ctxt:expr, $code:expr) => {{
        if unsafe { $code } == ::odpi::constants::DPI_SUCCESS as i32 {
            ()
        } else {
            return Err(::error::from_dpi_context($ctxt).into());
        }
    }};
}
