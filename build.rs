fn main() {
    #[cfg(feature = "generate")]
    {
        use std::path::Path;

        let bindings = bindgen::Builder::default()
            .use_core()
            .clang_arg("--no-standard-includes")
            .clang_arg("-I./devkitARM/arm-none-eabi/include")
            .clang_arg("-I./devkitARM/lib/gcc/arm-none-eabi/10.2.0/include")
            .clang_arg("-I./libctru/include")
            .parse_callbacks(Box::new(bindgen::CargoCallbacks))
            .header("libctru/include/3ds.h")
            .generate()
            .unwrap();

        let generated_path =
            Path::new(std::env::var("CARGO_MANIFEST_DIR").unwrap().as_str()).join("generated");

        let _ = std::fs::create_dir_all(&generated_path);

        bindings
            .write_to_file(generated_path.join("3ds.rs"))
            .expect("Couldn't write bindings!");
    }
}
