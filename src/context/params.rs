//!
// use odpi::flags;
use odpi::structs::ODPICommonCreateParams;

///
pub struct Create {
    ///
    #[allow(dead_code)]
    ccp: *mut ODPICommonCreateParams,
    // mode: flags::ODPICreateMode,
}

impl Create {
    ///
    pub fn new(ptr: *mut ODPICommonCreateParams) -> Create {
        Create { ccp: ptr }
    }
}
