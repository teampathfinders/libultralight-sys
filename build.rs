// fn main() {
//     let dst = cmake::Config::new("scripts")
//         .build_target("ALL_BUILD")
//         .build();
//
//     println!("cargo:rustc-link-search=native={}/build/SDK/lib", dst.display());
//     println!("cargo:rustc-link-lib=static=AppCore");
//     println!("cargo:rustc-link-lib=static=Ultralight");
//     println!("cargo:rustc-link-lib=static=UltralightCore");
//     println!("cargo:rustc-link-lib=static=WebCore");
//
//     let bindings = bindgen::Builder::default()
//         .clang_arg(format!("-F{}/build/SDK/include", dst.display()))
//         .header("wrapper.h")
//         .parse_callbacks(Box::new(bindgen::CargoCallbacks))
//         .generate()
//         .expect("Unable to generate bindings");
//
//     let out = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
//     bindings
//         .write_to_file(out.join("bindings.rs"))
//         .expect("Couldn't write Ultralight bindings to file");
// }

use std::env;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

fn main() {
    let dst = cmake::Config::new("scripts")
        .build_target("ALL_BUILD")
        .build();

    println!("cargo:rustc-link-search=native={}/build/SDK/lib", dst.display());
    println!("cargo:rustc-link-lib=static=AppCore");
    println!("cargo:rustc-link-lib=static=Ultralight");
    println!("cargo:rustc-link-lib=static=UltralightCore");
    println!("cargo:rustc-link-lib=static=WebCore");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-F{}/build/SDK/include", dst.display()))
        .impl_debug(true)
        .impl_partialeq(true)
        .generate_comments(true)
        .generate_inline_functions(true)
        .allowlist_var("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_type("^UL.*|JS.*|ul.*|WK.*")
        .allowlist_function("^UL.*|JS.*|ul.*|WK.*")
        .generate()
        .expect("Unable to generate bindings");

    bindings
        .write_to_file("src/lib.rs")
        .expect("Couldn't write bindings!");
}