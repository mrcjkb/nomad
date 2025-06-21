#![allow(missing_docs)]

use std::env;

fn main() {
    println!("cargo::rustc-check-cfg=cfg(headless)");

    if env::var("HEADLESS").as_deref() == Ok("true") {
        println!("cargo:rustc-cfg=headless");
    }
}
