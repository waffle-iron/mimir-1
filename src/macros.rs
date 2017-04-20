//! `oci` macros
macro_rules! try_dpi {
    ($ctxt:expr, $code:expr) => {{
        if unsafe { $code } == ::odpi::constants::DPI_SUCCESS {
            ()
        } else {
            return Err(::error::from_dpi_context($ctxt).into());
        }
    }};
}
