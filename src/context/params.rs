//!
use odpi::flags;
use odpi::structs::ODPICommonCreateParams;

///
pub struct Create {
    ///
    ccp: *mut ODPICommonCreateParams,
    /// Create Mode.
    mode: flags::ODPICreateMode,
}

impl Create {
    ///
    pub fn new(ptr: *mut ODPICommonCreateParams) -> Create {
        let mode = unsafe { (*ptr).create_mode };
        Create {
            ccp: ptr,
            mode: mode,
        }
    }

    ///
    pub fn get_mode(&self) -> flags::ODPICreateMode {
        self.mode
    }

    ///
    pub fn set_mode(&mut self, mode: flags::ODPICreateMode) -> &mut Create {
        self.mode = mode;
        self
    }
}
