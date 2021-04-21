fn main() {
    #[cfg(feature = "generate")]
    {
        use std::path::Path;

        let ctru = bindgen::Builder::default()
            .clang_arg("--no-standard-includes")
            .clang_arg("-I./devkitARM/arm-none-eabi/include")
            .clang_arg("-I./devkitARM/lib/gcc/arm-none-eabi/10.2.0/include")
            .clang_arg("-I./libctru/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .header("libctru/include/3ds.h")
            .header("libctru/include/citro2d.h")
            .generate()
            .unwrap();

        let generated_path =
            Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("generated");

        let _ = std::fs::create_dir_all(&generated_path);

        let ctru = ctru
            .to_string()
            .replace(
                "pub type size_t = ::std::os::raw::c_ulonglong;",
                "pub type size_t = usize;",
            )
            .replace(
                "pub type _ssize_t = ::std::os::raw::c_longlong;",
                "pub type _ssize_t = isize;",
            )
            .replace("pub type wchar_t = ::std::os::raw::c_ushort;", "");

        std::fs::write(generated_path.join("3ds.rs"), ctru).expect("Couldn't write bindings!");
    }
}
