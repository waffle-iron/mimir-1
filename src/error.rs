//! `oci` errors
use context::errorinfo::ErrorInfo;

error_chain! {
    foreign_links {
        Nul(::std::ffi::NulError);
        EnvVar(::std::env::VarError);
    }

    errors {
        Connection(fn_name: String) {
            description("Connection: call to ODPI-C function failed!")
            display("Connection: call to '{}' function failed!", fn_name)
        }
        Context(fn_name: String) {
            description("Context: call to ODPI-C function failed!")
            display("Context: call to '{}' function failed!", fn_name)
        }
        BranchId {
            description("The given batch id is longer than 64 bytes!")
            display("The given batch id is longer than 64 bytes!")
        }
        ContextCreateFailed {
            description("Failed to create the ODPI-C context!")
            display("Failed to create the ODPI-C context!")
        }
        DpiError(err: ErrorInfo) {
            description("ODPI-C Error")
            display("ODPI-C Error! {}", err)
        }
        OciError(err: ErrorInfo) {
            description("OCI Error!")
            display("OCI Error! {}", err)
        }
        Statement(fn_name: String) {
            description("Statement: call to ODPI-C function failed!")
            display("Statement: call to '{}' function failed!", fn_name)
        }
        TxnId {
            description("The given transaction id is longer than 64 bytes!")
            display("The given transaction id is longer than 64 bytes!")
        }
        Var(fn_name: String) {
            description("Var: call to ODPI-C function failed!")
            display("Var: call to '{}' function failed!", fn_name)
        }
    }
}
