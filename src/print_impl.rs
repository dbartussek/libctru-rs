use crate::raw::c_char;

extern "C" {
    #[doc(hidden)]
    pub fn puts(string: *const c_char);
}

#[macro_export]
macro_rules! print {
    () => ();
    ($($arg:tt)*) => (unsafe {
        $crate::print_impl::puts(
            alloc::format!(
                "{}\0",
                alloc::format!($($arg)*)
            ).as_ptr()
        );
    });
}

#[macro_export]
macro_rules! println {
    () => (unsafe {
        $crate::print_impl::puts("\n\0".as_ptr());
    });
    ($($arg:tt)*) => (unsafe {
        $crate::print_impl::puts(
            alloc::format!(
                "{}\n\0",
                alloc::format!($($arg)*)).as_ptr()
        );
    });
}
