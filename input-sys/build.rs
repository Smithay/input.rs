#[cfg(feature = "gen")]
extern crate bindgen;

#[cfg(feature = "gen")]
use std::env;
#[cfg(feature = "gen")]
use std::path::Path;

#[cfg(not(feature = "gen"))]
fn main() {}

#[cfg(feature = "gen")]
fn main() {
    // Setup bindings builder
    let mut builder = bindgen::builder();

    #[cfg(feature = "libinput_1_15")]
    {
        builder = builder.header("include/libinput.1.15.0.h");
    }
    #[cfg(all(feature = "libinput_1_14", not(feature = "libinput_1_15")))]
    {
        builder = builder.header("include/libinput.1.14.0.h");
    }
    #[cfg(all(
        feature = "libinput_1_11",
        not(any(feature = "libinput_1_14", feature = "libinput_1_15"))
    ))]
    {
        builder = builder.header("include/libinput.1.11.0.h");
    }
    #[cfg(not(any(
        feature = "libinput_1_11",
        feature = "libinput_1_14",
        feature = "libinput_1_15"
    )))]
    {
        builder = builder.header("include/libinput.1.9.0.h");
    }

    let generated = builder
        .ctypes_prefix("::libc")
        .whitelist_type(r"^libinput_.*$")
        .whitelist_function(r"^libinput_.*$")
        .rustfmt_bindings(false)
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed=include/");

    // Generate the bindings
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");

    generated.write_to_file(dest_path).unwrap();
}
