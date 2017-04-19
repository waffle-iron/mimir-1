macro_rules! try_dpi {
    ($ctxt:expr, $code:expr) => {{
        if unsafe { $code } == ::ffi::DPI_SUCCESS as i32 {
            ()
        } else {
            return Err(::error::error_from_dpi_context($ctxt).into());
        }
    }};
}
