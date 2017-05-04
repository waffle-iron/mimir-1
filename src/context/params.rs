//!
use odpi::flags;
use odpi::structs::{ODPICommonCreateParams, ODPIConnCreateParams};
use std::ffi::CStr;
use util::ODPIStr;

///
pub struct Create {
    ///
    ccp: ODPICommonCreateParams,
}

impl Create {
    ///
    pub fn new(ccp: ODPICommonCreateParams) -> Create {
        Create { ccp: ccp }
    }

    ///
    pub fn get_create_mode(&self) -> flags::ODPICreateMode {
        self.ccp.create_mode
    }

    ///
    pub fn set_create_mode(&mut self, create_mode: flags::ODPICreateMode) -> &mut Create {
        self.ccp.create_mode = create_mode;
        self
    }

    ///
    pub fn get_encoding(&self) -> String {
        let encoding_cstr = unsafe { CStr::from_ptr(self.ccp.encoding) };
        encoding_cstr.to_string_lossy().into_owned()
    }

    ///
    pub fn set_encoding(&mut self, encoding: *const ::std::os::raw::c_char) -> &mut Create {
        self.ccp.encoding = encoding;
        self
    }

    ///
    pub fn get_nchar_encoding(&self) -> String {
        let encoding_cstr = unsafe { CStr::from_ptr(self.ccp.encoding) };
        encoding_cstr.to_string_lossy().into_owned()
    }

    ///
    pub fn set_nchar_encoding(&mut self, encoding: *const ::std::os::raw::c_char) -> &mut Create {
        self.ccp.encoding = encoding;
        self
    }

    ///
    pub fn get_edition(&self) -> String {
        let edition_s = ODPIStr::new(self.ccp.edition, self.ccp.edition_length);
        edition_s.into()
    }

    ///
    pub fn set_edition(&mut self, edition: &str) -> &mut Create {
        let edition_s = ODPIStr::from(edition);
        self.ccp.edition = edition_s.ptr();
        self.ccp.edition_length = edition_s.len();
        self
    }

    ///
    pub fn get_driver_name(&self) -> String {
        let driver_name_s = ODPIStr::new(self.ccp.driver_name, self.ccp.driver_name_length);
        driver_name_s.into()
    }

    ///
    pub fn set_driver_name(&mut self, driver_name: &str) -> &mut Create {
        let driver_name_s = ODPIStr::from(driver_name);
        self.ccp.driver_name = driver_name_s.ptr();
        self.ccp.driver_name_length = driver_name_s.len();
        self
    }
}

///
pub struct Conn {
    ///
    conn: ODPIConnCreateParams,
}

impl Conn {
    ///
    pub fn new(conn: ODPIConnCreateParams) -> Conn {
        Conn { conn: conn }
    }

    ///
    pub fn get_auth_mode(&self) -> flags::ODPIAuthMode {
        self.conn.auth_mode
    }

    ///
    pub fn set_auth_mode(&mut self, auth_mode: flags::ODPIAuthMode) -> &mut Conn {
        self.conn.auth_mode = auth_mode;
        self
    }

    ///
    pub fn get_connection_class(&self) -> String {
        let connection_class_s = ODPIStr::new(self.conn.connection_class,
                                              self.conn.connection_class_length);
        connection_class_s.into()
    }

    ///
    pub fn set_connection_class(&mut self, connection_class: &str) -> &mut Conn {
        let connection_class_s = ODPIStr::from(connection_class);
        self.conn.connection_class = connection_class_s.ptr();
        self.conn.connection_class_length = connection_class_s.len();
        self
    }

    ///
    pub fn get_purity(&self) -> flags::ODPIPurity {
        self.conn.purity
    }

    ///
    pub fn set_purity(&mut self, purity: flags::ODPIPurity) -> &mut Conn {
        self.conn.purity = purity;
        self
    }
}
