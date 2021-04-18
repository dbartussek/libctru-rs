use std::path::PathBuf;

fn main() {
    let bindings = bindgen::Builder::default()
        .clang_arg("--no-standard-includes")
        .clang_arg("-I./devkitARM/arm-none-eabi/include")
        .clang_arg("-I./devkitARM/lib/gcc/arm-none-eabi/10.2.0/include")
        .clang_arg("-I./libctru/include")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .header("libctru/include/3ds.h")
        .generate()
        .unwrap();

    let out_path = PathBuf::from(std::env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("3ds.rs"))
        .expect("Couldn't write bindings!");
}
