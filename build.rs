#![deny(warnings)]

extern crate rustc_version;

use std::env;
use std::fs::File;
use std::io::Write;

fn build_info() -> String {
    let meta = rustc_version::version_meta();

    format!("pub const BUILD_INFO: &'static str = {{ \"{}: {} - Compiled with rust-{} ({:?} channel)\" }};",
            env::var("CARGO_PKG_VERSION").unwrap(),
            env::var("TARGET").unwrap(),
            meta.semver,
            meta.channel)
}

fn main() {
    File::create("src/build_info.rs").unwrap().write_all(build_info().as_bytes()).unwrap();
}
