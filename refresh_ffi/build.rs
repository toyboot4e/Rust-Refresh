//! Build script of `refresh-sys`

use {
    cmake::Config,
    std::{
        env,
        path::{Path, PathBuf},
    },
};

fn main() {
    println!("cargo:rerun-if-changed=wrappers");
    println!("cargo:rerun-if-changed=Refresh");

    self::compile();

    let root = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let args = &[format!("-I{}", root.join("Refresh/include").display())];
    let derive_default = true;

    self::gen_bindings(
        root.join("wrappers/refresh_ffi.h"),
        root.join("src/ffi.rs"),
        args,
        derive_default,
        "//! Refresh.h\n",
    );

    self::gen_bindings(
        root.join("wrappers/refresh_ffi_image.h"),
        root.join("src/img.rs"),
        args,
        derive_default,
        "//! Refresh_Image.h\n",
    );
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

/// Generates Rust FFI using a wrapper header file
fn gen_bindings(
    wrapper: impl AsRef<Path>,
    dst: impl AsRef<Path>,
    args: impl IntoIterator<Item = impl AsRef<str>>,
    derive_default: bool,
    docstring: &str,
) {
    let gen = bindgen::Builder::default()
        .header(format!("{}", wrapper.as_ref().display()))
        .derive_default(true)
        .clang_args(args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks));

    let gen = gen.raw_line(docstring);
    let gen = gen.raw_line(r"#![allow(warnings)]");
    let gen = gen.derive_default(derive_default);

    let gen = gen.generate().unwrap_or_else(|err| {
        panic!(
            "Unable to generate bindings for `{}`. Original error {:?}",
            dst.as_ref().display(),
            err
        )
    });

    // it's `ok` to fail on crates.io
    gen.write_to_file(dst).ok();
}
