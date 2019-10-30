extern crate bindgen;
extern crate cc;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("{:?}",cc::Build::new().get_compiler().path());
    cc::Build::new()
        .cpp(true)
        .warnings(true)
        .cpp_link_stdlib("stdc++")
        .flag("-Wall")
        .flag("-Wextra")
        .flag("-std=c++11")
        .flag("-g")
        .include("src/edlib/include")
        .file("src/edlib/src/edlib.cpp")
        .compile("libedlib.a");
    let bindings = bindgen::Builder::default()
        .header("src/edlib/include/edlib.h")
        .generate()
        .expect("Unable to generate bindings");
    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
