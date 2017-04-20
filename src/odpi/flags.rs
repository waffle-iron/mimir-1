//! ODPI-C bitflags.
bitflags! {
    #[repr(C)]
    /// This enumeration identifies the mode to use when authorizing connections to the database.
    pub flags ODPIAuthMode: u32 {
        /// Default value used when creating connections.
        const DPI_MODE_AUTH_DEFAULT = 0b00000000,
        /// Authenticates with SYSDBA access.
        const DPI_MODE_AUTH_SYSDBA  = 0b00000010,
        /// Authenticates with SYSOPER access.
        const DPI_MODE_AUTH_SYSOPER = 0b00000100,
        /// Used together with DPI_MODE_AUTH_SYSDBA or DPI_MODE_AUTH_SYSOPER to authenticate for
        /// certain administrative tasks (such as starting up or shutting down the database).
        const DPI_MODE_AUTH_PRELIM  = 0b00001000,
        /// Authenticates with SYSASM access.
        const DPI_MODE_AUTH_SYSASM  = 0b1000000000000000,
    }
}

bitflags! {
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

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// This enumeration identifies the types of events that can take place. The event type is part of
/// the messages that are sent to subscriptions.
pub enum ODPIEventType {
    /// Indicates that no event has taken place.
    No = 0,
    /// Indicates that a database is being started up.
    Startup = 1,
    /// Indicates that a database is being shut down.
    Shutdown = 2,
    /// Indicates that a database is being shut down.
    ShutdownAny = 3,
    /// Indicates that a database is being dropped.
    DropDB = 4,
    /// Indicates that a registered query is being deregistered.
    Dereg = 5,
    /// Indicates that an object change has taken place.
    ObjectChange = 6,
    /// Indicates that a query change has taken place.
    QueryChange = 7,
}

bitflags! {
    #[repr(C)]
    /// This enumeration identifies the types of operations that can take place during object change
    /// and query change notification. It is used both as a filter when determining which operations
    /// to consider when sending notifications as well as identifying the operation that took place
    /// on a particular table or row when a notification is sent. Multiple values can be OR'ed
    /// together to specify multiple types of operations at the same time.
    pub flags ODPIOpCode: u32 {
        /// Indicates that notifications should be sent for all operations on the table or query.
        const DPI_OPCODE_ALL_OPS  = 0b00000000,
        /// Indicates that all rows have been changed in the table or query (or too many rows were
        /// changed or row information was not requested).
        const DPI_OPCODE_ALL_ROWS = 0b00000001,
        /// Indicates that an insert operation has taken place in the table or query.
        const DPI_OPCODE_INSERT   = 0b00000010,
        /// Indicates that an update operation has taken place in the table or query.
        const DPI_OPCODE_UPDATE   = 0b00000100,
        /// Indicates that a delete operation has taken place in the table or query.
        const DPI_OPCODE_DELETE   = 0b00001000,
        /// Indicates that the registered table or query has been altered.
        const DPI_OPCODE_ALTER    = 0b00010000,
        /// Indicates that the registered table or query has been dropped.
        const DPI_OPCODE_DROP     = 0b00100000,
        /// An unknown operation has taken place.
        const DPI_OPCODE_UNKNOWN  = 0b01000000,
    }
}

bitflags! {
    #[repr(C)]
    /// This enumeration identifies the mode to use when getting sessions from a session pool.
    pub flags ODPIPoolGetMode: u32 {
        /// Specifies that the caller should block until a session is available from the pool.
        const DPI_MODE_POOL_GET_WAIT     = 0b00000000,
        /// Specifies that the caller should return immediately, regardless of whether a session is
        /// available in the pool. If a session is not available an error is returned.
        const DPI_MODE_POOL_GET_NOWAIT   = 0b00000001,
        /// Specifies that a new session should be created if all of the sessions in the pool are
        /// busy, even if this exceeds the maximum sessions allowable for the session pool (see
        /// `dpiPoolCreateParams.maxSessions`)
        const DPI_MODE_POOL_GET_FORCEGET = 0b00000010,
    }
}

bitflags! {
    #[repr(C)]
    /// This enumeration identifies the purity of the sessions that are acquired when using
    /// connection classes during connection creation.
    pub flags ODPIPurity: u32 {
        /// Default value used when creating connections.
        const DPI_PURITY_DEFAULT = 0b00000000,
        /// A connection is required that has not been tainted with any prior session state.
        const DPI_PURITY_NEW     = 0b00000001,
        /// A connection is permitted to have prior session state.
        const DPI_PURITY_SELF    = 0b00000010,
    }
}

bitflags! {
    #[repr(C)]
    /// This enumeration identifies the namespaces supported by subscriptions.
    pub flags ODPISubscrNamespace: u32 {
        /// Identifies the namespace used for receiving notifications for database object changes
        /// and query changes.
        const DPI_SUBSCR_NAMESPACE_DBCHANGE = 0b00000000,
    }
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
/// This enumeration identifies the protocol used for sending notifications to subscriptions.
pub enum ODPISubscrProtocol {
    /// Notifications are sent by calling the callback specified when the subscription was
    /// registered.
    Callback = 0,
    /// Notifications are sent by sending an e-mail to the e-mail address specified when the
    /// subscription was registered.
    Mail = 1,
    /// Notifications are sent by calling the PL/SQL procedure specified when the subscription was
    /// registered.
    PLSQL = 2,
    /// Notifications are sent to the URL specified when the subscription was registered.
    HTTP = 3,
}

bitflags! {
    #[repr(C)]
    /// This enumeration identifies the quality of service flags for sending notifications to
    /// subscriptions.
    pub flags ODPISubscrQOS: u32 {
        /// Notifications are sent reliably. If the database fails, the notifications are not lost.
        /// This is not supported for nonpersistent queues or buffered messaging.
        const DPI_SUBSCR_QOS_RELIABLE    = 0b00000001,
        /// When the notification has been received, the subscription is removed.
        const DPI_SUBSCR_QOS_DEREG_NFY   = 0b00000010,
        /// Information on the rows affected by the database or query change is sent along with the
        /// notification.
        const DPI_SUBSCR_QOS_ROWIDS      = 0b00000100,
        /// Perform query notification instead of database change notification. Notification is done
        /// in guaranteed mode which guarantees that the query has in fact changed.
        const DPI_SUBSCR_QOS_QUERY       = 0b00001000,
        /// Perform query notification in best effort mode which may result in notifications being
        /// sent when the query has not in fact changed. This is needed for complex queries that
        /// cannot be registered in guaranteed mode.
        const DPI_SUBSCR_QOS_BEST_EFFORT = 0b00010000,
    }
}
