//! This structure is used for returning Oracle version information about the Oracle Client
//! (`get_client_version()`) and Oracle Database (`get_server_version()`).
use odpi::structs::ODPIVersionInfo;
use std::fmt;

/// Wrapper for the `ODPIVersionInfo` structure.
pub struct Info {
    /// The version.
    version: String,
    /// The version number.
    version_num: u32,
    /// The release string.
    release: Option<String>,
}

impl Info {
    /// Get the `version` value.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the `version_num` value.
    pub fn version_num(&self) -> u32 {
        self.version_num
    }

    /// Get the `release` value.
    pub fn release(&self) -> &str {
        if let Some(ref release) = self.release {
            release
        } else {
            ""
        }
    }

    /// Set the `release` value.
    pub fn set_release(&mut self, release: Option<String>) -> &mut Info {
        self.release = release;
        self
    }
}

impl From<ODPIVersionInfo> for Info {
    fn from(ovi: ODPIVersionInfo) -> Info {
        let version = format!("{}.{}.{}.{}.{}",
                              ovi.version_num,
                              ovi.release_num,
                              ovi.update_num,
                              ovi.port_release_num,
                              ovi.port_update_num);
        Info {
            version: version,
            version_num: ovi.full_version_num,
            release: None,
        }
    }
}

impl fmt::Display for Info {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.version)?;

        if let Some(ref release) = self.release {
            writeln!(f, "{}", release)?;
        }

        Ok(())
    }
}
