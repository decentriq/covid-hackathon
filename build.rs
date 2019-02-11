use std::env;
use std::path::Path;
use std::process::Command;

use autotools::Config;

fn compile() {
    println!("cargo:rustc-link-lib=z");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dir = "vendor/proj-5.2.0";
    Command::new("autoreconf").arg("-fi").current_dir(dir).output().unwrap();
    Config::new(dir)
        .cflag("-fPIC")
        .build();
    println!("cargo:rustc-link-lib=static=proj");
    let dir = "vendor/geos-3.7.1";
    Command::new("autoreconf").arg("-fi").current_dir(dir).output().unwrap();
    Config::new(dir)
        .enable_shared()
        .cflag(format!("-I {}/include -fPIC", out_dir))
        .cxxflag("-fPIC")
        .ldflag(format!("-L{}/lib", out_dir))
        .build();
    println!("cargo:rustc-link-lib=static=geos");
    println!("cargo:rustc-link-lib=static=geos_c");
    let dir = "vendor/freexl-1.0.5";
    Command::new("autoreconf").arg("-fi").current_dir(dir).output().unwrap();
    Config::new(dir)
        .cflag("-fPIC")
        .build();
    println!("cargo:rustc-link-lib=static=freexl");
    let dir = "vendor/libspatialite-4.3.0a";
    Command::new("autoreconf").arg("-fi").current_dir(dir).output().unwrap();
    Config::new(dir)
        .cflag(format!("-I {}/include -fPIC", out_dir))
        .ldflag(format!("-L{}/lib -static-libstdc++ -static-libgcc", out_dir))
        .with("-geosconfig", Some(&format!("{}/bin/geos-config", out_dir)))
        .disable("-libxml2", None)
        .build();
    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=spatialite");
    let target  = env::var("TARGET").unwrap();
    if target.contains("apple") {
        println!("cargo:rustc-link-lib=dylib=c++");
    } else if target.contains("linux") {
        println!("cargo:rustc-link-lib=dylib=stdc++");
    } else {
        unimplemented!();
    }
}

fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();
    if has_pkgconfig {
        if pkg_config::find_library("spatialite").is_ok() {
            pkg_config::probe_library("spatialite").unwrap();
        } else {
            compile();
        }
    } else {
        compile();
    }
    let out_dir = env::var("OUT_DIR").unwrap();
    let _ = bindgen::builder()
        .header("src/wrapper.h")
        .clang_arg(format!("-I{}/include", out_dir))
        .generate().unwrap()
        .write_to_file(Path::new(&out_dir).join("spatialite_sys.rs"));
}
