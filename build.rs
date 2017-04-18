extern crate bindgen;

use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("odpi/include/dpi.h")
        .use_core()
        .generate()
        .unwrap()
        .write_to_file(Path::new(&out_dir).join("bindings.rs"));
}
