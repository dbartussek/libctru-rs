#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod raw {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/3ds.rs"));
}
