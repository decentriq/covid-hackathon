use std::env;
use std::path::Path;
use std::process::Command;

fn compile() {
    let out_dir = env::var("OUT_DIR").unwrap();
    autotools::build("vendor/proj-5.2.0");
    println!("cargo:rustc-link-lib=static=proj");
    autotools::Config::new("vendor/geos-3.7.1")
        .cflag(format!("-I {}/include", out_dir))
        .ldflag(format!("-L{}/lib", out_dir))
        .build();
    println!("cargo:rustc-link-lib=static=geos");
    autotools::build("vendor/freexl-1.0.5");
    println!("cargo:rustc-link-lib=static=freexl");
    let config = autotools::Config::new("vendor/libspatialite-4.3.0a")
        .cflag(format!("-I {}/include", out_dir))
        .ldflag(format!("-L{}/lib", out_dir))
        .with("-geosconfig", Some(&format!("{}/bin/geos-config", out_dir)))
        .disable("-libxml2", None)
        .build();
    println!("cargo:rustc-link-search=native={}/lib", config.display());
    println!("cargo:rustc-link-lib=static=spatialite");
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
