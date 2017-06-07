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
    let generated = bindgen::builder()
        .header("include/libinput.1.7.0.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .whitelisted_type(r"^libinput_.*$")
        .whitelisted_function(r"^libinput_.*$")
        .constified_enum("libinput_led")
        .constified_enum("libinput_config_send_events_mode")
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed=include/libinput.1.7.0.h");

    // Generate the bindings
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("gen.rs");

    generated.write_to_file(dest_path).unwrap();
}
