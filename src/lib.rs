#![feature(restricted_std)]
#![deny(missing_docs)]

//! Bindings to [libctru](https://libctru.devkitpro.org/), the user mode 3DS library.
//!
//! The libctru and devkitARM directories contain the C headers from which
//! generated/3ds.rs is produced by bindgen. As bindgen cannot produce no_std
//! compatible files on its own, 3ds.rs is modified by:
//! - In the original C header libctru/include/3ds/mii.h, the MiiData struct is
//!   declared as packed. This breaks bindgens generated code because Rust
//!   cannot handle a packed struct containing another with repr(align).
//! - Replacing ::std::os::raw::c_(someint) with drone_ctypes::c_(someint)
//! - Replacing size_t with an alias to drone_ctypes::c_ulong

/// Alloc is exported to allow print!/println! to be used
/// in dependent crates that don't use it.
pub extern crate alloc;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(missing_docs)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
pub mod raw {
    include!(concat!(env!("CARGO_MANIFEST_DIR"), "/generated/3ds.rs"));
}
