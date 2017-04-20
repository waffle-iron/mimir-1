//! ODPI-C bitflags.

bitflags!{
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

bitflags!{
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

bitflags!{
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
