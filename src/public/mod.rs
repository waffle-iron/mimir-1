//! Wrappers for ODPI-C Public Functions/Structs/Enums
pub mod connection;
pub mod context;
pub mod data;

use odpi::structs::ODPIVersionInfo;

/// Wrapper for the `ODPIVersionInfo` structure.
pub struct VersionInfo {
    /// The version.
    version: String,
    /// The version number.
    version_num: u32,
}

impl VersionInfo {
    /// Get the `version` value.
    pub fn version(&self) -> &str {
        &self.version
    }

    /// Get the `version_num` value.
    pub fn version_num(&self) -> u32 {
        self.version_num
    }
}

impl From<ODPIVersionInfo> for VersionInfo {
    fn from(ovi: ODPIVersionInfo) -> VersionInfo {
        let version = format!("{}.{}.{}.{}.{}",
                              ovi.version_num,
                              ovi.release_num,
                              ovi.update_num,
                              ovi.port_release_num,
                              ovi.port_update_num);
        VersionInfo {
            version: version,
            version_num: ovi.full_version_num,
        }

    }
}
