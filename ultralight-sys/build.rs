use std::{env, path::PathBuf};

fn main() {
    // env::set_var("LIBCLANG_PATH", r"D:\misc\clang\bin");

    println!("cargo:rerun-if-changed=./include");
    println!("cargo:rerun-if-changed=build.rs");

    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=dylib=AppCore");
        println!("cargo:rustc-link-lib=dylib=Ultralight");
        println!("cargo:rustc-link-lib=dylib=WebCore");
    } else {
        println!("cargo:rustc-link-lib=AppCore");
        println!("cargo:rustc-link-lib=Ultralight");
        println!("cargo:rustc-link-lib=WebCore");
    }

    let mut bindings = bindgen::Builder::default()
        .header("include/wrapper.h")
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_var("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_type("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_function("^UL.*|JS.*|ul.*|WK.*");

    match env::var("ULTRALIGHT_SDK_PATH").map(PathBuf::from) {
        Ok(p) => {
            let lib = p.join("lib");
            println!("cargo:rustc-link-search={}", lib.to_string_lossy());
            let bin = p.join("bin");
            println!("cargo:rustc-link-search={}", bin.to_string_lossy());
            let include = p.join("include");
            bindings = bindings.clang_arg(format!("-I{}", include.to_string_lossy()));
        }
        Err(_) => {
            println!("cargo:warning=ULTRALIGHT_SDK_PATH not found, set it first");
            println!("cargo:rustc-link-search=/usr/local/lib/");
        }
    }

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .generate()
        .expect("Unable to generate bindings")
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
