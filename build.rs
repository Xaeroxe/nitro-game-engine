use std::env;
use std::path::PathBuf;

fn main() {
    let target = env::var("TARGET").unwrap();
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut lib_dir = manifest_dir.clone();
    let mut dll_dir = manifest_dir.clone();
    if target == "x86_64-pc-windows-msvc" {
        lib_dir.push("msvc");
        lib_dir.push("lib");
        lib_dir.push("64");
        dll_dir.push("msvc");
        dll_dir.push("dll");
        dll_dir.push("64");
    }
    else if target == "x86_64-pc-windows-gnu" {
        lib_dir.push("gnu-mingw");
        lib_dir.push("lib");
        lib_dir.push("64");
        dll_dir.push("gnu-mingw");
        dll_dir.push("dll");
        dll_dir.push("64");
    }
    else if target == "i686-pc-windows-msvc" {
        lib_dir.push("msvc");
        lib_dir.push("lib");
        lib_dir.push("32");
        dll_dir.push("msvc");
        dll_dir.push("dll");
        dll_dir.push("32");
    }
    else if target == "i686-pc-windows-gnu" {
        lib_dir.push("gnu-mingw");
        lib_dir.push("lib");
        lib_dir.push("32");
        dll_dir.push("gnu-mingw");
        dll_dir.push("dll");
        dll_dir.push("32");
    }
    if target.contains("pc-windows") {
        println!("cargo:rustc-link-search=all={}", lib_dir.display());
        for entry in std::fs::read_dir(dll_dir).expect("Can't read DLL dir")  {
            let entry_path = entry.expect("Invalid fs entry").path();
            let file_name_result = entry_path.file_name();
            let mut new_file_path = manifest_dir.clone();
            if let Some(file_name) = file_name_result {
                new_file_path.push(file_name.to_str().unwrap());
                std::fs::copy(&entry_path, new_file_path.as_path()).expect("Can't copy from DLL dir");
            }
        }
    }
}
