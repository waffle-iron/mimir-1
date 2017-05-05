//!
use odpi::flags;
use odpi::structs::{ODPIAppContext, ODPICommonCreateParams, ODPIConnCreateParams,
                    ODPIPoolCreateParams};
use pool::Pool;
use std::ffi::CStr;
use util::ODPIStr;

/// This structure is used for passing application context to the database during the process of
/// creating standalone connections. These values are ignored when acquiring a connection from a
/// session pool or when using DRCP (Database Resident Connection Pooling).
pub struct AppContext {
    /// The ODPI-C dpiAppContext struct.
    ctxt: ODPIAppContext,
}

impl AppContext {
    /// Create a new `AppContext` struct.
    pub fn new(namespace: &str, name: &str, value: &str) -> AppContext {
        let namespace_s = ODPIStr::from(namespace);
        let name_s = ODPIStr::from(name);
        let value_s = ODPIStr::from(value);

        let ctxt = ODPIAppContext {
            namespace_name: namespace_s.ptr(),
            namespace_name_length: namespace_s.len(),
            name: name_s.ptr(),
            name_length: name_s.len(),
            value: value_s.ptr(),
            value_length: value_s.len(),
        };

        AppContext { ctxt: ctxt }
    }

    /// Create a new `AppContext` struct from an ODPI-C dpiAppContext struct.
    pub fn from_odpi(ctxt: ODPIAppContext) -> AppContext {
        AppContext { ctxt: ctxt }
    }

    /// Get the `namespace_name` value.
    ///
    /// Specifies the value of the "namespace" parameter to sys_context(). It is expected to be a
    /// byte string in the encoding specified in the `ODPIConnCreateParams` structure and must not
    /// be NULL.
    pub fn get_namespace_name(&self) -> String {
        let namespace_name_s = ODPIStr::new(self.ctxt.namespace_name,
                                            self.ctxt.namespace_name_length);
        namespace_name_s.into()
    }

    /// Get the `name` value.
    ///
    /// Specifies the value of the "parameter" parameter to sys_context(). It is expected to be a
    /// byte string in the encoding specified in the `ODPIConnCreateParams` structure and must not
    /// be NULL.
    pub fn get_name(&self) -> String {
        let name_s = ODPIStr::new(self.ctxt.name, self.ctxt.name_length);
        name_s.into()
    }

    /// Get the `value` value.
    ///
    /// Specifies the value that will be returned from sys_context(). It is expected to be a byte
    /// string in the encoding specified in the `ODPIConnCreateParams` structure and must not be
    /// NULL.
    pub fn get_value(&self) -> String {
        let value_s = ODPIStr::new(self.ctxt.value, self.ctxt.value_length);
        value_s.into()
    }
}

/// This structure is used for creating session pools and standalone connections to the database.
pub struct CommonCreate {
    /// The ODPI-C dpiCommonCreateParams struct.
    ccp: ODPICommonCreateParams,
}

impl CommonCreate {
    /// Create a new `Create` struct.
    pub fn new(ccp: ODPICommonCreateParams) -> CommonCreate {
        CommonCreate { ccp: ccp }
    }

    /// Get the `create_mode` value.
    ///
    /// Specifies the mode used for creating connections. It is expected to be one or more of the
    /// values from the enumeration `ODPICreateMode`, OR'ed together. The default value is
    /// DPI_MODE_CREATE_DEFAULT.
    pub fn get_create_mode(&self) -> flags::ODPICreateMode {
        self.ccp.create_mode
    }

    /// Set the `create_mode` value.
    pub fn set_create_mode(&mut self, create_mode: flags::ODPICreateMode) -> &mut CommonCreate {
        self.ccp.create_mode = create_mode;
        self
    }

    /// Get the `encoding` value.
    ///
    /// Specifies the encoding to use for CHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_LANG environment variable. The default value is NULL.
    pub fn get_encoding(&self) -> String {
        let encoding_cstr = unsafe { CStr::from_ptr(self.ccp.encoding) };
        encoding_cstr.to_string_lossy().into_owned()
    }

    /// Set the `encoding` value.
    pub fn set_encoding(&mut self, encoding: *const ::std::os::raw::c_char) -> &mut CommonCreate {
        self.ccp.encoding = encoding;
        self
    }

    /// Get the `nchar_encoding` value.
    ///
    /// Specifies the encoding to use for NCHAR data, as a null-terminated ASCII string. Either an
    /// IANA or Oracle specific character set name is expected. NULL is also acceptable which
    /// implies the use of the NLS_NCHAR environment variable. The default value is NULL.
    pub fn get_nchar_encoding(&self) -> String {
        let encoding_cstr = unsafe { CStr::from_ptr(self.ccp.nchar_encoding) };
        encoding_cstr.to_string_lossy().into_owned()
    }

    /// Set the `nchar_encoding` value.
    pub fn set_nchar_encoding(&mut self,
                              nchar_encoding: *const ::std::os::raw::c_char)
                              -> &mut CommonCreate {
        self.ccp.nchar_encoding = nchar_encoding;
        self
    }

    /// Get the `edition` value.
    ///
    /// Specifies the edition to be used when creating a standalone connection. It is expected to be
    /// NULL (meaning that no edition is set) or a byte string in the encoding specified by the
    /// `encoding` member. The default value is NULL.
    pub fn get_edition(&self) -> String {
        let edition_s = ODPIStr::new(self.ccp.edition, self.ccp.edition_length);
        edition_s.into()
    }

    /// Set the `edition` value.
    pub fn set_edition(&mut self, edition: &str) -> &mut CommonCreate {
        let edition_s = ODPIStr::from(edition);
        self.ccp.edition = edition_s.ptr();
        self.ccp.edition_length = edition_s.len();
        self
    }

    /// Get the `driver_name` value.
    ///
    /// Specifies the name of the driver that is being used. It is expected to be NULL or a byte
    /// string in the encoding specified by the dpiCommonCreateParams.encoding member. The default
    /// value is NULL.
    pub fn get_driver_name(&self) -> String {
        let driver_name_s = ODPIStr::new(self.ccp.driver_name, self.ccp.driver_name_length);
        driver_name_s.into()
    }

    /// Set the `driver_name` value.
    pub fn set_driver_name(&mut self, driver_name: &str) -> &mut CommonCreate {
        let driver_name_s = ODPIStr::from(driver_name);
        self.ccp.driver_name = driver_name_s.ptr();
        self.ccp.driver_name_length = driver_name_s.len();
        self
    }
}

/// This structure is used for creating connections to the database, whether standalone or acquired
/// from a session pool. All members are initialized to default values using the
/// `init_common_conn_params` function. Care should be taken to ensure a copy of this structure
/// exists only as long as needed to create the connection since it can contain a clear text copy of
/// credentials used for connecting to the database.
pub struct ConnCreate {
    /// The ODPI-C dpiConnCreateParams struct.
    conn: ODPIConnCreateParams,
}

impl ConnCreate {
    /// Create a new `ConnCreate` struct.
    pub fn new(conn: ODPIConnCreateParams) -> ConnCreate {
        ConnCreate { conn: conn }
    }

    /// Get the `auth_mode` value.
    ///
    /// Specifies the mode used for authorizing connections. It is expected to be one or more of the
    /// values from the enumeration `ODPIAuthMode`, OR'ed together. The default value is
    /// DPI_MODE_AUTH_DEFAULT.
    pub fn get_auth_mode(&self) -> flags::ODPIAuthMode {
        self.conn.auth_mode
    }

    /// Set the `auth_mode` value.
    pub fn set_auth_mode(&mut self, auth_mode: flags::ODPIAuthMode) -> &mut ConnCreate {
        self.conn.auth_mode = auth_mode;
        self
    }

    /// Get the `connection_class` value.
    ///
    /// Specifies the connection class to use when connecting to the database. This is used with
    /// DRCP (database resident connection pooling) or to further subdivide a session pool. It is
    /// expected to be NULL (meaning that no connection class will be set) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub fn get_connection_class(&self) -> String {
        let connection_class_s = ODPIStr::new(self.conn.connection_class,
                                              self.conn.connection_class_length);
        connection_class_s.into()
    }

    /// Set the `connection_class` value.
    pub fn set_connection_class(&mut self, connection_class: &str) -> &mut ConnCreate {
        let connection_class_s = ODPIStr::from(connection_class);
        self.conn.connection_class = connection_class_s.ptr();
        self.conn.connection_class_length = connection_class_s.len();
        self
    }

    /// Get the `purity` value.
    ///
    /// Specifies the level of purity required when creating a connection using a connection class.
    /// It is expected to be one of the values from the enumeration `ODPIPurity`. The default value
    /// is DPI_PURITY_DEFAULT.
    pub fn get_purity(&self) -> flags::ODPIPurity {
        self.conn.purity
    }

    /// Set the `purity` value.
    pub fn set_purity(&mut self, purity: flags::ODPIPurity) -> &mut ConnCreate {
        self.conn.purity = purity;
        self
    }

    /// Get the `new_password` value. (Do we need to expose this?)
    ///
    /// Specifies the new password to set when creating a connection. This value is only used when
    /// creating a standalone connection. It is expected to be NULL or a byte string in the encoding
    ///  used for CHAR data. The default value of this member is NULL. If specified, the password
    /// for the user is changed when the connection is created (useful when the password has expired
    /// and a session cannot be established without changing the password).
    pub fn get_new_password(&self) -> String {
        let new_password_s = ODPIStr::new(self.conn.new_password, self.conn.new_password_length);
        new_password_s.into()
    }

    /// Set the `new_password` value.
    pub fn set_new_password(&mut self, new_password: &str) -> &mut ConnCreate {
        let new_password_s = ODPIStr::from(new_password);
        self.conn.new_password = new_password_s.ptr();
        self.conn.new_password_length = new_password_s.len();
        self
    }

    /// Get the `app_context` value.
    ///
    /// Specifies the application context that will be set when the connection is created. This
    /// value is only used when creating standalone connections. It is expected to be NULL or an
    /// array of `ODPIAppContext` structures. The context specified here can be used in logon
    /// triggers, for example. The default value is NULL.
    pub fn get_app_context(&self) -> Vec<AppContext> {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_wrap))]
        let len = self.conn.num_app_context as isize;
        let head_ptr = self.conn.app_context;

        let mut app_contexts = Vec::new();
        for i in 0..len {
            app_contexts.push(AppContext::from_odpi(unsafe { *head_ptr.offset(i) }));
        }
        app_contexts
    }

    /// Set the `app_context` value.
    pub fn set_app_context(&mut self, app_contexts: Vec<AppContext>) -> &mut ConnCreate {
        #[cfg_attr(feature = "cargo-clippy", allow(cast_possible_truncation))]
        let len = app_contexts.len() as u32;
        let mut oac_vec: Vec<ODPIAppContext> = Vec::new();
        for ac in &app_contexts {
            oac_vec.push(ac.ctxt);
        }
        let ac_ptr = app_contexts.as_ptr();
        self.conn.app_context = ac_ptr as *mut ODPIAppContext;
        self.conn.num_app_context = len;
        self
    }

    /// Get the `num_app_context` value.
    ///
    /// Specifies the number of elements found in the dpiConnCreateParams.appContext member. The
    /// default value is 0.
    pub fn get_num_app_context(&self) -> u32 {
        self.conn.num_app_context
    }

    /// Get the `external_auth` value.
    ///
    /// Specifies whether external authentication should be used to create the connection. If this
    /// value is 0, the user name and password values must be specified in the call to
    /// `ConnCreateection.create` otherwise, the user name and password values must be zero length
    /// orNULL. The default value is 0.
    pub fn get_external_auth(&self) -> i32 {
        self.conn.external_auth
    }

    /// Set the `external_auth` value.
    pub fn set_external_auth(&mut self, external_auth: i32) -> &mut ConnCreate {
        self.conn.external_auth = external_auth;
        self
    }

    /// Get the `external_handle` value.
    ///
    /// Specifies an OCI service context handle created externally that will be used instead of
    /// creating a connection. The default value is NULL.
    pub fn get_external_handle(&self) -> *mut ::std::os::raw::c_void {
        self.conn.external_handle
    }

    /// Set the `external_handle` value.
    pub fn set_external_handle(&mut self,
                               external_handle: *mut ::std::os::raw::c_void)
                               -> &mut ConnCreate {
        self.conn.external_handle = external_handle;
        self
    }

    /// Get the `pool` value.
    ///
    /// Specifies the session pool from which to acquire a connection or NULL if a standalone
    /// connection should be created. The default value is NULL.
    pub fn get_pool(&self) -> Pool {
        Pool::from_odpi(self.conn.pool)
    }

    /// Set the `pool` value.
    pub fn set_pool(&mut self, pool: Pool) -> &mut ConnCreate {
        self.conn.pool = pool.get_pool();
        self
    }

    /// Get the `tag` value.
    ///
    /// Specifies the tag to use when acquiring a connection from a session pool. This member is
    /// ignored when creating a standalone connection. If specified, the tag restricts the type of
    /// session that can be returned to those with that tag or a NULL tag. If the member
    /// `match_any_tag` is set, however, any session can be returned if no matching sessions are
    /// found.
    ///
    /// The value is expected to be NULL (any session can be returned) or a byte string in the
    /// encoding used for CHAR data. The default value is NULL.
    pub fn get_tag(&self) -> String {
        let tag_s = ODPIStr::new(self.conn.tag, self.conn.tag_length);
        tag_s.into()
    }

    /// Set the `tag` value.
    pub fn set_tag(&mut self, tag: &str) -> &mut ConnCreate {
        let tag_s = ODPIStr::from(tag);
        self.conn.tag = tag_s.ptr();
        self.conn.tag_length = tag_s.len();
        self
    }

    /// Get the `match_any_tag` value.
    ///
    /// Specifies whether any tagged session should be accepted when acquiring a connection from a
    /// session pool, if no connection using the tag specified in the `tag` is available. This value
    /// is only used when acquiring a connection from a session pool. The default value is false.
    pub fn get_match_any_tag(&self) -> bool {
        self.conn.match_any_tag == 1
    }

    /// Set the `match_any_tag` value.
    pub fn set_match_any_tag(&mut self, match_any_tag: bool) -> &mut ConnCreate {
        self.conn.match_any_tag = if match_any_tag { 1 } else { 0 };
        self
    }

    /// Get the `out_tag` value.
    ///
    /// Specifies the tag of the connection that was acquired from a session pool, or NULL if the
    /// session was not tagged. This member is left untouched when creating a standalone connection
    /// and is filled in only if the connection acquired from the session pool was tagged. If filled
    /// in, it is a byte string in the encoding used for CHAR data.
    pub fn get_out_tag(&self) -> String {
        if self.conn.out_tag.is_null() {
            "".to_string()
        } else {
            let res = ODPIStr::new(self.conn.out_tag, self.conn.out_tag_length);
            res.into()
        }
    }

    /// Get the `out_tag_found` value.
    pub fn get_out_tag_found(&self) -> bool {
        self.conn.out_tag_found != 0
    }
}

/// This structure is used for creating session pools, which can in turn be used to create
/// connections that are acquired from that session pool. All members are initialized to default
/// values using the `init_pool_create_params()` function.
pub struct PoolCreate {
    /// The ODPI-C dpiPoolCreateParams struct.
    pool: ODPIPoolCreateParams,
}

impl PoolCreate {
    /// Create a new `PoolCreate` struct.
    pub fn new(pool: ODPIPoolCreateParams) -> PoolCreate {
        PoolCreate { pool: pool }
    }

    /// Get the `min_sessions` value.
    ///
    /// Specifies the minimum number of sessions to be created by the session pool. This value is
    /// ignored if the `homogeneous` member has a value of 0. The default value is 1.
    pub fn get_min_sessions(&self) -> u32 {
        self.pool.min_sessions
    }

    /// Set the `min_sessions` value.
    pub fn set_min_sessions(&mut self, min_sessions: u32) -> &mut PoolCreate {
        self.pool.min_sessions = min_sessions;
        self
    }

    /// Get the `max_sessions` value.
    ///
    /// Specifies the maximum number of sessions that can be created by the session pool. Values of
    /// and higher are acceptable. The default value is 1.
    pub fn get_max_sessions(&self) -> u32 {
        self.pool.max_sessions
    }

    /// Set the `max_sessions` value.
    pub fn set_max_sessions(&mut self, max_sessions: u32) -> &mut PoolCreate {
        self.pool.max_sessions = max_sessions;
        self
    }

    /// Get the `session_increment` value.
    ///
    /// Specifies the number of sessions that will be created by the session pool when more sessions
    /// are required and the number of sessions is less than the maximum allowed. This value is
    /// ignored if the `homogeneous` member has a value of 0. This value added to the `min_sessions`
    /// member value must not exceed the `max_sessions` member value. The default value is 0.
    pub fn get_session_increment(&self) -> u32 {
        self.pool.session_increment
    }

    /// Set the `session_increment` value.
    pub fn set_session_increment(&mut self, session_increment: u32) -> &mut PoolCreate {
        self.pool.session_increment = session_increment;
        self
    }

    /// Get the `ping_interval` value.
    ///
    /// Specifies the number of seconds since a connection has last been used before a ping will be
    /// performed to verify that the connection is still valid. A negative value disables this
    /// check. The default value is 60. This value is ignored in clients 12.2 and later since a much
    /// faster internal check is done by the Oracle client.
    pub fn get_ping_interval(&self) -> i32 {
        self.pool.ping_interval
    }

    /// Set the `ping_interval` value.
    pub fn set_ping_interval(&mut self, ping_interval: i32) -> &mut PoolCreate {
        self.pool.ping_interval = ping_interval;
        self
    }

    /// Get the `ping_timeout` value.
    ///
    /// Specifies the number of milliseconds to wait when performing a ping to verify the connection
    /// is still valid before the connection is considered invalid and is dropped. The default value
    /// is 5000 (5 seconds). This value is ignored in clients 12.2 and later since a much faster
    /// internal check is done by the Oracle client.
    pub fn get_ping_timeout(&self) -> i32 {
        self.pool.ping_timeout
    }

    /// Set the `ping_timeout` value.
    pub fn set_ping_timeout(&mut self, ping_timeout: i32) -> &mut PoolCreate {
        self.pool.ping_timeout = ping_timeout;
        self
    }

    /// Get the `homogeneous` value.
    ///
    /// Specifies whether the pool is homogeneous or not. In a homogeneous pool all connections use
    /// the same credentials whereas in a heterogeneous pool other credentials are permitted. The
    /// default value is true.
    pub fn get_homogeneous(&self) -> bool {
        self.pool.homogeneous == 1
    }

    /// Set the `homogeneous` value.
    pub fn set_homogeneous(&mut self, homogeneous: bool) -> &mut PoolCreate {
        self.pool.homogeneous = if homogeneous { 1 } else { 0 };
        self
    }

    /// Get the `external_auth` value.
    ///
    /// Specifies whether external authentication should be used to create the sessions in the pool.
    /// If this value is 0, the user name and password values must be specified in the call to \
    /// `create()` otherwise, the user name and password values must be zero length or NULL. The
    /// default value is 0.
    pub fn get_external_auth(&self) -> bool {
        self.pool.external_auth == 1
    }

    /// Set the `external_auth` value.
    pub fn set_external_auth(&mut self, external_auth: bool) -> &mut PoolCreate {
        self.pool.external_auth = if external_auth { 1 } else { 0 };
        self
    }

    /// Get the `get_mode` value.
    ///
    /// Specifies the mode to use when sessions are acquired from the pool. It is expected to be one
    /// of the values from the enumeration `ODPIPoolGetMode`. The default value is
    /// DPI_MODE_POOL_GET_NOWAIT
    pub fn get_get_mode(&self) -> flags::ODPIPoolGetMode {
        self.pool.get_mode
    }

    /// Set the `get_mode` value.
    pub fn set_get_mode(&mut self, get_mode: flags::ODPIPoolGetMode) -> &mut PoolCreate {
        self.pool.get_mode = get_mode;
        self
    }

    /// Get the `out_pool_name` value.
    ///
    /// This member is populated upon successful creation of a pool using the function `create()`.
    /// It is a byte string in the encoding used for CHAR data. Any value specified prior to
    /// creating the session pool is ignored.
    pub fn get_out_pool_name(&self) -> String {
        if self.pool.out_pool_name.is_null() {
            "".to_string()
        } else {
            let res = ODPIStr::new(self.pool.out_pool_name, self.pool.out_pool_name_length);
            res.into()
        }
    }
}
