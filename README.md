# libctru

Bindings to [libctru](https://libctru.devkitpro.org/), the user mode 3DS library.

The libctru and devkitARM directories contain the C headers from which
generated/3ds.rs is produced by bindgen. As bindgen cannot produce no_std
compatible files on its own, 3ds.rs is modified by:
- In the original C header libctru/include/3ds/mii.h, the MiiData struct is
  declared as packed. This breaks bindgens generated code because Rust
  cannot handle a packed struct containing another with repr(align).
- Replacing ::std::os::raw::c_(someint) with drone_ctypes::c_(someint)
- Replacing size_t with an alias to drone_ctypes::c_ulong
