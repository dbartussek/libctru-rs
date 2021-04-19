use crate::raw::c_char;

extern "C" {
    #[doc(hidden)]
    fn putchar(c: c_char);

    #[doc(hidden)]
    pub fn puts(string: *const c_char);
}

#[doc(hidden)]
pub fn put_str(s: &str) {
    unsafe {
        for c in s.as_bytes() {
            putchar(*c);
        }
    }
}

/// Prints to the console without appending a newline.
///
/// A console can be created by calling `consoleInit`.
/// [See the C examples for now.](https://github.com/devkitPro/3ds-examples/blob/master/graphics/printing/hello-world/source/main.c)
#[macro_export]
macro_rules! print {
    () => ();
    ($($arg:tt)*) => ({
        $crate::print_impl::put_str(
            $crate::alloc::format!($($arg)*).as_str()
        );
    });
}

/// Prints to the console with a newline.
///
/// A console can be created by calling `consoleInit`.
/// [See the C examples for now.](https://github.com/devkitPro/3ds-examples/blob/master/graphics/printing/hello-world/source/main.c)
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
