//! ODPI-C Public Structs.
use libc::{c_char, c_int, c_void};
use odpi::{flags, opaque};
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct ODPIAppContext {
    pub namespace_name: *const c_char,
    pub namespace_name_length: u32,
    pub name: *const c_char,
    pub name_length: u32,
    pub value: *const c_char,
    pub value_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used when creating session pools and standalone connections to the
/// database.
pub struct ODPICommonCreateParams {
    /// The mode to use when creating connections to the database.
    pub create_mode: flags::ODPICreateMode,
    /// Specifies the encoding to use for CHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_LANG environment variable. The default value is NULL.
    pub encoding: *const c_char,
    /// Specifies the encoding to use for NCHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_NCHAR environment variable. The default value is NULL.
    pub nchar_encoding: *const c_char,
    /// Specifies the edition to be used when creating a standalone connection. It is expected to
    /// be NULL (meaning that no edition is set) or a byte string in the encoding specified by the
    /// dpiCommonCreateParams.encoding member. The default value is NULL.
    pub edition: *const c_char,
    /// Specifies the length of the dpiCommonCreateParams.edition member, in bytes. The default
    /// value is 0.
    pub edition_length: u32,
    /// Specifies the name of the driver that is being used. It is expected to be NULL or a byte
    /// string in the encoding specified by the dpiCommonCreateParams.encoding member. The default
    /// value is NULL.
    pub driver_name: *const c_char,
    /// Specifies the length of the dpiCommonCreateParams.driverName member, in bytes. The default
    /// value is 0.
    pub driver_name_length: u32,
}

impl Default for ODPICommonCreateParams {
    fn default() -> ODPICommonCreateParams {
        ODPICommonCreateParams {
            create_mode: flags::DPI_MODE_CREATE_DEFAULT,
            encoding: ptr::null(),
            nchar_encoding: ptr::null(),
            edition: ptr::null(),
            edition_length: 0,
            driver_name: ptr::null(),
            driver_name_length: 0,
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating connections to the database, whether standalone or acquired
/// from a session pool. Care should be taken to ensure a copy of this structure exists only as long
/// as needed to create the connection since it can contain a clear text copy of credentials used
/// for connecting to the database.
pub struct ODPIConnCreateParams {
    /// Specifies the mode used for authorizing connections. It is expected to be one or more of the
    /// values from the enumeration `ODPIAuthMode`, OR'ed together. The default value is
    /// DPI_MODE_AUTH_DEFAULT.
    pub auth_mode: flags::ODPIAuthMode,
    /// Specifies the connection class to use when connecting to the database. This is used with
    /// DRCP (database resident connection pooling) or to further subdivide a session pool. It is
    /// expected to be NULL (meaning that no connection class will be set) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub connection_class: *const c_char,
    /// Specifies the length of the dpiConnCreateParams.connectionClass member, in bytes. The
    /// default value is 0.
    pub connection_class_length: u32,
    /// Specifies the level of purity required when creating a connection using a connection class.
    /// It is expected to be one of the values from the enumeration `ODPIPurity`. The default value
    /// is DPI_PURITY_DEFAULT.
    pub purity: flags::ODPIPurity,
    /// Specifies the new password to set when creating a connection. This value is only used when
    /// creating a standalone connection. It is expected to be NULL or a byte string in the encoding
    /// used for CHAR data. The default value of this member is NULL. If specified, the password
    /// for the user is changed when the connection is created (useful when the password has expired
    /// and a session cannot be established without changing the password).
    pub new_password: *const c_char,
    /// Specifies the length of the dpiConnCreateParams.newPassword member, in bytes. The default
    /// value is 0.
    pub new_password_length: u32,
    /// Specifies the application context that will be set when the connection is created. This
    /// value is only used when creating standalone connections. It is expected to be NULL or an
    /// array of `ODPIAppContext` structures. The context specified here can be used in logon
    /// triggers, for example. The default value is NULL.
    pub app_context: *mut ODPIAppContext,
    /// Specifies the number of elements found in the dpiConnCreateParams.appContext member. The
    /// default value is 0.
    pub num_app_context: u32,
    /// Specifies whether external authentication should be used to create the connection. If this
    /// value is 0, the user name and password values must be specified in the call to
    /// `dpiConn_create()`; otherwise, the user name and password values must be zero length or NULL
    /// The default value is 0.
    pub external_auth: c_int,
    /// Specifies an OCI service context handle created externally that will be used instead of
    /// creating a connection. The default value is NULL.
    pub external_handle: *mut c_void,
    /// Specifies the session pool from which to acquire a connection or NULL if a standalone
    /// connection should be created. The default value is NULL.
    pub pool: *mut opaque::ODPIPool,
    /// Specifies the tag to use when acquiring a connection from a session pool. This member is
    /// ignored when creating a standalone connection. If specified, the tag restricts the type of
    /// session that can be returned to those with that tag or a NULL tag. If the member
    /// `dpiConnCreateParams.matchAnyTag` is set, however, any session can be returned if no
    /// matching sessions are found.
    ///
    /// The value is expected to be NULL (any session can be returned) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub tag: *const c_char,
    /// Specifies the length of the dpiConnCreateParams.tag member, in bytes. The default value is
    /// 0.
    pub tag_length: u32,
    /// Specifies whether any tagged session should be accepted when acquiring a connection from a
    /// session pool, if no connection using the tag specified in the dpiConnCreateParams.tag is
    /// available. This value is only used when acquiring a connection from a session pool. The
    /// default value is 0.
    pub match_any_tag: c_int,
    /// Specifies the tag of the connection that was acquired from a session pool, or NULL if the
    /// session was not tagged. This member is left untouched when creating a standalone connection
    /// and is filled in only if the connection acquired from the session pool was tagged. If filled
    /// in, it is a byte string in the encoding used for CHAR data.
    pub out_tag: *const c_char,
    /// Specifies the length of the dpiConnCreateParams.outTag member, in bytes.
    pub out_tag_length: u32,
    /// Specifies if the connection created used the tag specified by the dpiConnCreateParams.tag
    /// member. It is only filled in if the connection was acquired from a session pool and a tag
    /// was initially specified.
    pub out_tag_found: c_int,
}

impl Default for ODPIConnCreateParams {
    fn default() -> ODPIConnCreateParams {
        ODPIConnCreateParams {
            auth_mode: flags::DPI_MODE_AUTH_DEFAULT,
            connection_class: ptr::null(),
            connection_class_length: 0,
            purity: flags::DPI_PURITY_DEFAULT,
            new_password: ptr::null(),
            new_password_length: 0,
            app_context: ptr::null_mut(),
            num_app_context: 0,
            external_auth: 0,
            external_handle: ptr::null_mut(),
            pool: ptr::null_mut(),
            tag: ptr::null(),
            tag_length: 0,
            match_any_tag: 0,
            out_tag: ptr::null(),
            out_tag_length: 0,
            out_tag_found: 0,
        }
    }
}
