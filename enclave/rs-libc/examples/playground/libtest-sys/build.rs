use std::path::Path;
use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let out_path = Path::new(&out_dir).join("bindgen.rs");
    build_bundled::main(&out_dir, &out_path)
}

mod build_bundled {
    use cc;
    use std::path::Path;

    pub fn main(out_dir: &str, out_path: &Path) {
        use super::{bindings, HeaderLocation};
        let header = HeaderLocation::FromPath("testlib/test.h".to_owned());
        bindings::write_to_out_dir(header, out_path);

        let mut cfg = cc::Build::new();
        cfg.file("testlib/test.c");
        cfg.flag("-nostdlib")
           .flag("-fno-stack-protector")
           .flag("-U_FORTIFY_SOURCE")
           .define("_FORTIFY_SOURCE", Some("0"))
           .compile("libtest.a");
        println!("cargo:lib_dir={}", out_dir);
    }
}

pub enum HeaderLocation {
    FromPath(String),
}

impl From<HeaderLocation> for String {
    fn from(header: HeaderLocation) -> String {
        match header {
            HeaderLocation::FromPath(path) => path,
        }
    }
}

mod bindings {
    use bindgen;

    use super::HeaderLocation;
    use bindgen::callbacks::{IntKind, ParseCallbacks};

    use std::fs::OpenOptions;
    use std::io::Write;
    use std::path::Path;

    #[derive(Debug)]
    struct LibTypeChooser;

    impl ParseCallbacks for LibTypeChooser {
        fn int_macro(&self, _name: &str, value: i64) -> Option<IntKind> {
            if value >= i32::min_value() as i64 && value <= i32::max_value() as i64 {
                Some(IntKind::I32)
            } else {
                None
            }
        }
    }

    pub fn write_to_out_dir(header: HeaderLocation, out_path: &Path) {
        let header: String = header.into();
        let mut output = Vec::new();
        let bindings = bindgen::builder()
            .header(header.clone())
            .parse_callbacks(Box::new(LibTypeChooser))
            .rustfmt_bindings(true);
        bindings
            .generate()
            .expect(&format!("could not run bindgen on header {}", header))
            .write(Box::new(&mut output))
            .expect("could not write output of bindgen");
        let output = String::from_utf8(output).expect("bindgen output was not UTF-8?!");

        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(out_path.clone())
            .expect(&format!("Could not write to {:?}", out_path));

        file.write_all(output.as_bytes())
            .expect(&format!("Could not write to {:?}", out_path));
    }
}

