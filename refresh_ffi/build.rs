//! Build script of `refresh-sys`

use {
    cmake::Config,
    std::{
        env,
        path::{Path, PathBuf},
    },
};

fn main() {
    self::compile();
    self::gen_bindings("wrappers/refresh_ffi.h", "ffi.rs");
    self::gen_bindings("wrappers/refresh_ffi_image.h", "img.rs");
}

/// Run `cmake` (only when it's necessary) and link the output library
fn compile() {
    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let out_lib_path = out_dir.join("libRefresh.dylib");
    if !out_lib_path.is_file() {
        let path = root.join("Refresh");
        let _out = Config::new(path)
            .no_build_target(true)
            .cflag("-w") // suppress errors
            .build();
    }

    println!(
        "cargo:rustc-link-search=native={}",
        out_dir.join("build").display()
    );
    println!("cargo:rustc-link-lib=dylib=Refresh");
}

/// Generates bindings using a wrapper header file
fn gen_bindings(wrapper: impl AsRef<Path>, dst_file_name: impl AsRef<Path>) {
    let wrapper = wrapper.as_ref();
    let dst_file_name = dst_file_name.as_ref();

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = root.join("src");
    let dst = out_dir.join(&dst_file_name);

    let gen = bindgen::Builder::default()
        .header(format!("{}", wrapper.display()))
        .derive_default(true)
        .clang_arg(format!("-I{}", root.join("Refresh/include").display()))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let gen = gen.raw_line(r"#![allow(warnings)]");

    let gen = gen.generate().unwrap_or_else(|err| {
        panic!(
            "Unable to generate bindings for `{}`. Original error {:?}",
            dst_file_name.display(),
            err
        )
    });

    // it's `ok` to fail conidering crates.io
    gen.write_to_file(&dst).ok();
}
