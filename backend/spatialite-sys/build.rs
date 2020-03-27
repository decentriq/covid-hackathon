use std::env;
use std::path::Path;
use std::process::Command;

use autotools::Config;

fn maybe_cross_compile(config: &mut Config) {
    if let Some(cc) = env::var("RUST_ANDROID_GRADLE_CC").ok() {
        config.env("CC", cc.clone());
        config.env("CXX", format!("{}++", cc));
    }
    if let Some(ld) = env::var("RUSTC_LINKER").ok() {
        config.env("LD", ld);
    }
    config.build();
}

fn compile() {
    println!("cargo:rustc-link-lib=z");
    let out_dir = env::var("OUT_DIR").unwrap();
    let dir = "vendor/sqlite-autoconf-3270100";
    let mut config = Config::new(dir);
    config.reconf("-fi");
    config.cflag("-fPIC");
    maybe_cross_compile(&mut config);
    println!("cargo:rustc-link-lib=static=sqlite3");
    let dir = "vendor/proj-5.2.0";
    let mut config = Config::new(dir);
    config.reconf("-fi");
    config.cflag(format!("-I {}/include -fPIC", out_dir));
    config.cxxflag(format!("-I {}/include -fPIC", out_dir));
    config.ldflag(format!("-L{}/lib", out_dir));
    maybe_cross_compile(&mut config);
    println!("cargo:rustc-link-lib=static=proj");
    let dir = "vendor/geos-3.7.1";
    let mut config = Config::new(dir);
    config.reconf("-fi");
    config.cflag(format!("-I {}/include -fPIC", out_dir));
    config.cxxflag("-fPIC");
    config.ldflag(format!("-L{}/lib", out_dir));
    maybe_cross_compile(&mut config);
    println!("cargo:rustc-link-lib=static=geos");
    println!("cargo:rustc-link-lib=static=geos_c");
    let target  = env::var("TARGET").unwrap();
    // let dir = "vendor/libiconv-1.15";
    // let mut config = Config::new(dir);
    // config.cflag(format!("-I {}/include -fPIC", out_dir));
    // config.ldflag(format!("-L{}/lib", out_dir));;
    // maybe_cross_compile(&mut config);
    // println!("cargo:rustc-link-lib=static=iconv");
    let dir = "vendor/libspatialite-4.3.0a";
    let mut config = Config::new(dir);
    config.reconf("-fi");
    config.disable("examples", None);
    config.disable("freexl", None);
    config.disable("iconv", None);
    config.cflag(format!("-pthread -I {}/include -fPIC", out_dir));
    config.ldflag(format!("-L{}/lib -latomic -ldl", out_dir));
    config.with("geosconfig", Some(&format!("{}/bin/geos-config", out_dir)));
    config.disable("libxml2", None);
    maybe_cross_compile(&mut config);
    println!("cargo:rustc-link-search={}/lib", out_dir);
    println!("cargo:rustc-link-lib=static=spatialite");
    if target.contains("android") || target.contains("apple") {
        println!("cargo:rustc-link-lib=static-nobundle=c++");
    } else {
        println!("cargo:rustc-link-lib=static-nobundle=stdc++");
    }
}

fn main() {
    println!("xxx");
    for (k, v) in env::vars() {
        println!("{} = {}", k, v);
    }
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
}
