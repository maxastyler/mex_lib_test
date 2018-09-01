use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    // println!("cargo:rustc-link-lib=bz2");
    // println!("cargo:-C target-feature=-crt-static");

    // println!(r"cargo:rustc-link-search=/home/max/matlab/bin/glnxa64");
    // println!(r"cargo:rustc-link-search=C:\Program Files\MATLAB\R2018a\extern\lib\win64\microsoft");
    // println!(r"cargo:rustc-link-lib=libmex");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    // let bindings = bindgen::Builder::default()
    // The input header we would like to generate
    // bindings for.
        // .header("wrapper.h")
    // Finish the builder and generate the bindings.
        // .clang_arg("-IC:\\Program Files\\MATLAB\\R2018a\\extern\\include")
        // .clang_arg("-I/home/max/matlab/extern/include")
        // .blacklist_function("mexFunction")
        // .generate()
    // Unwrap the Result and panic on failure.
        // .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");
}
