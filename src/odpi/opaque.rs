//! ODPI-C opaque structs
#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents connections to the database and is available by handle to a calling
/// application or driver.
pub struct ODPIConn([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents the context in which all activity in the library takes place.
pub struct ODPIContext([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents the available options for dequeuing messages when using advanced
/// queuing and is available by handle to a calling application or driver.
pub struct ODPIDeqOptions([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents the available options for enqueuing messages when using advanced
/// queuing and is available by handle to a calling application or driver.
pub struct ODPIEnqOptions([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents the available properties for messages when using advanced queuing and
/// is available by handle to a calling application or driver.
pub struct ODPIMsgProps([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents instances of the types created by the SQL command CREATE OR REPLACE
/// TYPE and is available by handle to a calling application or driver.
pub struct ODPIObject([u8; 0]);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure represents session pools and is available by handle to a calling application or
/// driver.
pub struct ODPIPool([u8; 0]);
