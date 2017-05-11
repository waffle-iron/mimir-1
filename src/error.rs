// Copyright (c) 2017 oic developers
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! `oci` errors
use common::error::Info;

error_chain! {
    foreign_links {
        Nul(::std::ffi::NulError);
        EnvVar(::std::env::VarError);
    }

    errors {
        BranchId {
            description("The given batch id is longer than 64 bytes!")
            display("The given batch id is longer than 64 bytes!")
        }
        Connection(fn_name: String) {
            description("Connection: call to ODPI-C function failed!")
            display("Connection: call to '{}' function failed!", fn_name)
        }
        Context(fn_name: String) {
            description("Context: call to ODPI-C function failed!")
            display("Context: call to '{}' function failed!", fn_name)
        }
        ContextCreateFailed {
            description("Failed to create the ODPI-C context!")
            display("Failed to create the ODPI-C context!")
        }
        DeqOptions(fn_name: String) {
            description("DeqOptions: call to ODPI-C function failed!")
            display("DeqOptions: call to '{}' function failed!", fn_name)
        }
        DpiError(err: Info) {
            description("ODPI-C Error")
            display("ODPI-C Error! {}", err)
        }
        EnqOptions(fn_name: String) {
            description("EnqOptions: call to ODPI-C function failed!")
            display("EnqOptions: call to '{}' function failed!", fn_name)
        }
        Lob(fn_name: String) {
            description("LOB: call to ODPI-C function failed!")
            display("LOB: call to '{}' function failed!", fn_name)
        }
        MsgProps(fn_name: String) {
            description("Context: call to ODPI-C function failed!")
            display("Context: call to '{}' function failed!", fn_name)
        }
        ObjectType(fn_name: String) {
            description("MsgProps: call to ODPI-C function failed!")
            display("MsgProps: call to '{}' function failed!", fn_name)
        }
        OciError(err: Info) {
            description("OCI Error!")
            display("OCI Error! {}", err)
        }
        Pool(fn_name: String) {
            description("Pool: call to ODPI-C function failed!")
            display("Pool: call to '{}' function failed!", fn_name)
        }
        Statement(fn_name: String) {
            description("Statement: call to ODPI-C function failed!")
            display("Statement: call to '{}' function failed!", fn_name)
        }
        Subscription(fn_name: String) {
            description("Subscription: call to ODPI-C function failed!")
            display("Subscription: call to '{}' function failed!", fn_name)
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
