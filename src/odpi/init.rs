//! `oic` ODPI-C Initialization Params
use libc::c_char;
use std::ptr;

bitflags!{
    #[repr(C)]
    /// This enumeration identifies the mode to use when creating connections to the database. Note
    /// that the OCI objects mode is always enabled.
    pub flags ODPICreateMode: u32 {
        /// Default value used when creating connections.
        const DPI_MODE_CREATE_DEFAULT  = 0b00000000,
        /// Enables threaded mode. Internal OCI structures not exposed to the user are protected
        /// from concurrent access by multiple threads. Error information is also managed in thread
        /// local storage.
        const DPI_MODE_CREATE_THREADED = 0b00000001,
        /// Enables events mode which is required for the use of advanced queuing (AQ) and
        /// continuous query notification (CQN).
        const DPI_MODE_CREATE_EVENTS   = 0b00000100,
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used when creating session pools and standalone connections to the
/// database.
pub struct ODPICommonCreateParams {
    pub create_mode: ODPICreateMode,
    pub encoding: *const c_char,
    pub nchar_encoding: *const c_char,
    pub edition: *const c_char,
    pub edition_length: u32,
    pub driver_name: *const c_char,
    pub driver_name_length: u32,
}

impl Default for ODPICommonCreateParams {
    fn default() -> ODPICommonCreateParams {
        ODPICommonCreateParams {
            create_mode: DPI_MODE_CREATE_DEFAULT,
            encoding: ptr::null(),
            nchar_encoding: ptr::null(),
            edition: ptr::null(),
            edition_length: 0,
            driver_name: ptr::null(),
            driver_name_length: 0,
        }
    }
}
