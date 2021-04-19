#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![no_std]

extern crate alloc;

pub mod raw {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/3ds.rs"));
}

mod allocator;
pub mod print_impl;

pub use crate::allocator::Mallocator;

pub mod prelude {
    pub use crate::{allocator::Mallocator, print, println};
}
