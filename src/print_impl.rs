use crate::raw::c_char;

extern "C" {
    #[doc(hidden)]
    pub fn printf(formatting: *const c_char, string: *const c_char);

    #[doc(hidden)]
    pub fn puts(string: *const c_char);
}

#[macro_export]
macro_rules! print {
    () => ();
    ($($arg:tt)*) => (unsafe {
        $crate::print_impl::printf(
            "%s",
            $crate::alloc::format!(
                "{}\0",
                $crate::alloc::format!($($arg)*)
            ).as_ptr()
        );
    });
}

#[macro_export]
macro_rules! println {
    () => (unsafe {
        $crate::print_impl::puts("\0".as_ptr());
    });
    ($($arg:tt)*) => (unsafe {
        $crate::print_impl::puts(
            $crate::alloc::format!(
                "{}\0",
                $crate::alloc::format!($($arg)*)).as_ptr()
        );
    });
}
