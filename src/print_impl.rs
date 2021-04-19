use crate::raw::c_char;

extern "C" {
    #[doc(hidden)]
    fn putchar(c: c_char);

    #[doc(hidden)]
    pub fn puts(string: *const c_char);
}

pub fn put_str(s: &str) {
    unsafe {
        for c in s.as_bytes() {
            putchar(*c);
        }
    }
}

#[macro_export]
macro_rules! print {
    () => ();
    ($($arg:tt)*) => ({
        $crate::print_impl::put_str(
            $crate::alloc::format!($($arg)*).as_str()
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
