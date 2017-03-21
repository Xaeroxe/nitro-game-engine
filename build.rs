use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    if target.contains("pc-windows") {
        let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        let mut lib_dir = manifest_dir.clone();
        lib_dir.push("lib");
        if target.contains("msvc") {
            lib_dir.push("msvc");
        }
        else {
            lib_dir.push("gnu-mingw");
        }
        if target.contains("x86_64") {
            lib_dir.push("64");
        }
        else {
            lib_dir.push("32");
        }
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
    }
}
