//! ODPI-C Public Structs.
use libc::{c_char, c_int, c_void};
use odpi::{externs, flags, opaque};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing application context to the database during the process of
/// creating standalone connections. These values are ignored when acquiring a connection from a
/// session pool or when using DRCP (Database Resident Connection Pooling). All values must be set
/// to valid values prior to being used in the `ODPIConnCreateParams` structure and must remain
/// valid until the execution of `dpiConn_create()` completes. Values set using this structure are
/// available in logon triggers by using the `sys_context()` SQL function.
pub struct ODPIAppContext {
    /// Specifies the value of the "namespace" parameter to sys_context(). It is expected to be a
    /// byte string in the encoding specified in the dpiConnCreateParams structure and must not be
    /// NULL.
    pub namespace_name: *const c_char,
    /// Specifies the length of the dpiAppContext.namespaceName member, in bytes.
    pub namespace_name_length: u32,
    /// Specifies the value of the "parameter" parameter to sys_context(). It is expected to be a
    /// byte string in the encoding specified in the dpiConnCreateParams structure and must not be
    /// NULL.
    pub name: *const c_char,
    /// Specifies the length of the dpiAppContext.name member, in bytes.
    pub name_length: u32,
    /// Specifies the value that will be returned from sys_context(). It is expected to be a byte
    /// string in the encoding specified in the dpiConnCreateParams structure and must not be NULL.
    pub value: *const c_char,
    /// Specifies the length of the dpiAppContext.value member, in bytes.
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

#[repr(C)]
#[derive(Copy, Clone, Debug)]
/// This structure is used for transferring error information from ODPI-C. All of the strings
/// referenced here may become invalid as soon as the next ODPI-C call is made.
pub struct ODPIErrorInfo {
    /// The OCI error code if an OCI error has taken place. If no OCI error has taken place the
    /// value is 0.
    pub code: i32,
    /// The parse error offset (in bytes) when executing a statement or the row offset when fetching
    /// batch error information. If neither of these cases are true, the value is 0.
    pub offset: u16,
    /// The error message as a byte string in the encoding specified by the dpiErrorInfo.encoding
    /// member.
    pub message: *const c_char,
    /// The length of the dpiErrorInfo.message member, in bytes.
    pub message_length: u32,
    /// The encoding in which the error message is encoded as a null-terminated string. For OCI
    /// errors this is the CHAR encoding used when the connection was created. For ODPI-C specific
    /// errors this is UTF-8.
    pub encoding: *const c_char,
    /// The public ODPI-C function name which was called in which the error took place. This is a
    /// null-terminated ASCII string.
    pub fn_name: *const c_char,
    /// The internal action that was being performed when the error took place. This is a
    /// null-terminated ASCII string.
    pub action: *const c_char,
    /// The SQLSTATE code associated with the error. This is a 5 character null-terminated string.
    pub sql_state: *const c_char,
    /// A boolean value indicating if the error is recoverable. This member always has a value of 0
    /// unless both client and server are at release 12.1 or higher.
    pub is_recoverable: c_int,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating session pools, which can in turn be used to create
/// connections that are acquired from that session pool.
pub struct ODPIPoolCreateParams {
    /// Specifies the minimum number of sessions to be created by the session pool. This value is
    /// ignored if the dpiPoolCreateParams.homogeneous member has a value of 0. The default value
    /// is 1.
    pub min_sessions: u32,
    /// Specifies the maximum number of sessions that can be created by the session pool. Values of
    /// 1 and higher are acceptable. The default value is 1.
    pub max_sessions: u32,
    /// Specifies the number of sessions that will be created by the session pool when more sessions
    /// are required and the number of sessions is less than the maximum allowed. This value is
    /// ignored if the dpiPoolCreateParams.homogeneous member has a value of 0. This value added to
    /// the dpiPoolCreateParams.minSessions member value must not exceed the
    /// dpiPoolCreateParams.maxSessions member value. The default value is 0.
    pub session_increment: u32,
    /// Specifies the number of seconds since a connection has last been used before a ping will be
    /// performed to verify that the connection is still valid. A negative value disables this
    /// check. The default value is 60. This value is ignored in clients 12.2 and later since a much
    /// faster internal check is done by the Oracle client.
    pub ping_interval: c_int,
    /// Specifies the number of milliseconds to wait when performing a ping to verify the connection
    /// is still valid before the connection is considered invalid and is dropped. The default value
    /// is 5000 (5 seconds). This value is ignored in clients 12.2 and later since a much faster
    /// internal check is done by the Oracle client.
    pub ping_timeout: c_int,
    /// Specifies whether the pool is homogeneous or not. In a homogeneous pool all connections use
    /// the same credentials whereas in a heterogeneous pool other credentials are permitted. The
    /// default value is 1.
    pub homogeneous: c_int,
    /// Specifies whether external authentication should be used to create the sessions in the pool.
    /// If this value is 0, the user name and password values must be specified in the call to
    /// dpiPool_create(); otherwise, the user name and password values must be zero length or NULL.
    /// The default value is 0.
    pub external_auth: c_int,
    /// Specifies the mode to use when sessions are acquired from the pool. It is expected to be one
    /// of the values from the enumeration `ODPIGetPoolMode`. The default value is
    /// DPI_MODE_POOL_GET_NOWAIT
    pub get_mode: flags::ODPIPoolGetMode,
    /// This member is populated upon successful creation of a pool using the function
    /// dpiPool_create(). It is a byte string in the encoding used for CHAR data. Any value
    /// specified prior to creating the session pool is ignored.
    pub out_pool_name: *const c_char,
    /// This member is populated upon successful creation of a pool using the function
    /// dpiPool_create(). It is the length of the dpiPoolCreateParams.outPoolName member, in bytes.
    /// Any value specified prior to creating the session pool is ignored.
    pub out_pool_name_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for creating subscriptions to messages sent for object change
/// notification, query change notification or advanced queuing.
pub struct ODPISubscrCreateParams {
    /// Specifies the namespace in which the subscription is created. It is expected to be one of
    /// the values from the enumeration `ODPISubscrNamespace`. The default value is
    /// DPI_SUBSCR_NAMESPACE_DBCHANGE.
    pub subscr_namespace: flags::ODPISubscrNamespace,
    /// Specifies the protocol used for sending notifications for the subscription. It is expected
    /// to be one of the values from the enumeration `ODPISubscrProtocol`. The default value is
    /// DPI_SUBSCR_PROTO_CALLBACK.
    pub protocol: flags::ODPISubscrProtocol,
    /// Specifies the quality of service flags to use with the subscription. It is expected to be
    /// one or more of the values from the enumeration `ODPISubscrQOS`, OR'ed together. The default
    /// value is to have no flags set.
    pub qos: flags::ODPISubscrQOS,
    /// Specifies which operations on the registered tables or queries should result in
    /// notifications. It is expected to be one or more of the values from the enumeration
    /// `ODPIOpCode`, OR'ed together. The default value is DPI_OPCODE_ALL_OPS.
    pub operations: flags::ODPIOpCode,
    /// Specifies the port number on which to receive notifications. The default value is 0, which
    /// means that a port number will be selected by the Oracle client.
    pub port_number: u32,
    /// Specifies the length of time, in seconds, before the subscription is unregistered. If the
    /// value is 0, the subscription remains active until explicitly unregistered. The default value
    /// is 0.
    pub timeout: u32,
    /// Specifies the name of the subscription, as a byte string in the encoding used for CHAR data.
    /// This name must be consistent with the namespace identified in the
    /// dpiSubscrCreateParams.subscrNamespace member. The default value is NULL.
    pub name: *const c_char,
    /// Specifies the length of the dpiSubscrCreateParams.name member, in bytes. The default value
    /// is 0.
    pub name_length: u32,
    /// Specifies the callback that will be called when a notification is sent to the subscription,
    /// if the dpiSubscrCreateParams.protocol member is set to DPI_SUBSCR_PROTO_CALLBACK. The
    /// callback accepts the following arguments:
    ///
    /// * context -- the value of the dpiSubscrCreateParams.callbackContext member.
    /// * message -- a pointer to the message that is being sent. The message is in the form
    ///                `ODPISubscrMessage`.
    ///
    /// The default value is NULL. If a callback is specified and a notification is sent, this will
    /// be performed on a separate thread. If database operations are going to take place, ensure
    /// that the create mode DPI_MODE_CREATE_THREADED is set in the structure dpiCommonCreateParams
    /// when creating the session pool or standalone connection that will be used in this callback.
    pub callback: externs::ODPISubscrCallback,
    /// Specifies the value that will be used as the first argument to the callback specified in the
    /// dpiSubscrCreateParams.callback member. The default value is NULL.
    pub callback_context: *mut c_void,
    /// Specifies the name of the recipient to which notifications are sent when the
    /// dpiSubscrCreateParams.protocol member is not set to DPI_SUBSCR_PROTO_CALLBACK. The value is
    /// expected to be a byte string in the encoding used for CHAR data. The default value is NULL.
    pub recipient_name: *const c_char,
    /// Specifies the length of the dpiSubscrCreateParams.recipientName member, in bytes. The
    /// default value is 0.
    pub recipient_name_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing messages sent by notifications to subscriptions. It is the
/// second parameter to the callback method specified in the `ODPISubscrCreateParams` structure.
pub struct ODPISubscrMessage {
    /// Specifies the type of event that took place which generated the notification. It will be one
    /// of the values from the enumeration `ODPIEventType`.
    pub event_type: flags::ODPIEventType,
    /// Specifies the name of the database which generated the notification, as a byte string in the
    /// encoding used for CHAR data.
    pub db_name: *const c_char,
    /// Specifies the length of the dpiSubscrMessage.dbName member, in bytes.
    pub db_name_length: u32,
    /// Specifies a pointer to an array of `ODPISubscrMessageTable` structures representing the list
    /// of tables that were modified and generated this notification. This value will be NULL if the
    /// value of the dpiSubscrMessage.eventType member is not equal to DPI_EVENT_OBJCHANGE.
    pub tables: *mut ODPISubscrMessageTable,
    /// Specifies the number of structures available in the dpiSubscrMessage.tables member.
    pub num_tables: u32,
    /// Specifies a pointer to an array of dpiSubscrMessageQuery structures representing the list of
    /// queries that were modified and generated this notification. This value will be NULL if the
    /// value of the dpiSubscrMessage.eventType member is not equal to DPI_EVENT_QUERYCHANGE.
    // pub queries: *mut dpiSubscrMessageQuery,
    /// Specifies the number of structures available in the dpiSubscrMessage.queries member.
    pub num_queries: u32,
    /// Specifies a pointer to a dpiErrorInfo structure. This value will be NULL if no error has
    /// taken place. If this value is not NULL the other members in this structure may not contain
    /// valid values.
    pub error_info: *mut ODPIErrorInfo,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information on the rows that were changed and resulted in the
/// notification message of which this structure is a part.
pub struct ODPISubscrMessageRow {
    /// Specifies the operations that took place on the registered query. It will be one or more of
    /// the values from the enumeration `ODPIOpCode`, OR'ed together.
    pub operation: flags::ODPIOpCode,
    /// Specifies the rowid of the row that was changed, in the encoding used for CHAR data.
    pub rowid: *const c_char,
    /// Specifies the length of the dpiSubscrMessageRow.rowid member, in bytes.
    pub rowid_length: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for passing information on the tables that were changed and resulted in
/// the notification message of which this structure is a part.
pub struct ODPISubscrMessageTable {
    /// Specifies the operations that took place on the modified table. It will be one or more of
    /// the values from the enumeration `ODPIOpCode`, OR'ed together.
    pub operation: flags::ODPIOpCode,
    /// Specifies the name of the table that was changed, in the encoding used for CHAR data.
    pub name: *const c_char,
    /// Specifies the length of the dpiSubscrMessageRow.name member, in bytes.
    pub name_length: u32,
    /// Specifies a pointer to an array of `ODPISubscrMessageRow` structures representing the list
    /// of rows that were modified by the event which generated this notification.
    pub rows: *mut ODPISubscrMessageRow,
    /// Specifies the number of structures available in the dpiSubscrMessageTable.rows member.
    pub num_rows: u32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
/// This structure is used for returning Oracle version information about the Oracle Client.
pub struct ODPIVersionInfo {
    /// Specifies the major version of the Oracle Client or Database.
    pub version_num: c_int,
    /// Specifies the release version of the Oracle Client or Database.
    pub release_num: c_int,
    /// Specifies the update version of the Oracle Client or Database.
    pub update_num: c_int,
    /// Specifies the port specific release version of the Oracle Client or Database.
    pub port_release_num: c_int,
    /// Specifies the port specific update version of the Oracle Client or Database.
    pub port_update_num: c_int,
    /// Specifies the full version (all five components) as a number that is suitable for
    /// comparison with the result of the macro DPI_ORACLE_VERSION_TO_NUMBER.
    pub full_version_num: u32,
}
