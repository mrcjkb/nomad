#![allow(missing_docs)]

fn main() {
    // On macOS we need to set these linker flags or nvim-oxi won't build.
    //
    // See https://github.com/rust-lang/rust/issues/62874 for more infos.
    #[cfg(target_os = "macos")]
    {
        println!("cargo:rustc-cdylib-link-arg=-undefined");
        println!("cargo:rustc-cdylib-link-arg=dynamic_lookup");
    }
}
