#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;
#[cfg(not(feature = "rust_codegen"))]
use cyclonedds_sys::DDSGenType;
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// if rust_codegen is not enabled, the allocators are generated
// in a different file
#[cfg(not(feature = "rust_codegen"))]
include!(concat!(env!("OUT_DIR"), "/DdsAllocator_impl.rs"));


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
