// use std::env;
// use std::path::PathBuf;

fn main() {
    println!("{:?}", cc::Build::new().get_compiler().path());
    if cfg!(target_os = "linux") {
        cc::Build::new()
            .cpp(true)
            .warnings(true)
            .cpp_link_stdlib(Some("stdc++"))
            .flag("-Wall")
            .flag("-Wextra")
            .flag("-O3")
            //.flag("-std=c++11")
            .flag("-g")
            .include("src/edlib/include")
            .file("src/edlib/src/edlib.cpp")
            .compile("libedlib.a");
    }
    if cfg!(target_os = "macos") {
        cc::Build::new()
            .cpp(true)
            .warnings(true)
            .cpp_link_stdlib(Some("c++"))
            .flag("-Wall")
            .flag("-Wextra")
            // .flag("-std=c++11")
            .flag("-O3")
            .flag("-g")
            .include("src/edlib/include")
            .file("src/edlib/src/edlib.cpp")
            .compile("edlib.lib");
    }
    if cfg!(target_os = "windows") {
        cc::Build::new()
            .cpp(true)
            .warnings(true)
            .cpp_link_stdlib(None)
            .flag("-Wall")
            // .flag("-std=c++11")
            .flag("-O3")
            .flag("-g")
            .include("src/edlib/include")
            .file("src/edlib/src/edlib.cpp")
            .compile("libedlib.a");
    }
}
