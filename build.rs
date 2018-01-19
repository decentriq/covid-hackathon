extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();
    if has_pkgconfig {
        if pkg_config::find_library("libgit2").is_ok() {
            pkg_config::probe_library("spatialite").unwrap();
        } else {
            println!("cargo:rustc-link-lib=spatialite");
        }
    } else {
        println!("cargo:rustc-link-lib=spatialite");
    }
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("src/wrapper.h")
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("spatialite_sys.rs"));
}
