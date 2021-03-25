#[cfg(feature = "gen")]
extern crate bindgen;

use std::env;

const LIB_VERSIONS: &[(u8, u8, u8)] = &[(1, 15, 0), (1, 14, 0), (1, 11, 0), (1, 9, 0)];

fn lib_version() -> &'static (u8, u8, u8) {
    for version in LIB_VERSIONS {
        if env::var(format!(
            "CARGO_FEATURE_LIBINPUT_{}_{}",
            version.0, version.1
        ))
        .is_ok()
        {
            return version;
        }
    }
    panic!("No libinput_<version> feature is set.");
}

#[cfg(not(feature = "gen"))]
fn main() {
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    if matches!(target_os.as_str(), "linux") {
        println!("cargo:rustc-env=LIBINPUT_TARGET_OS={}", target_os);
    } else {
        panic!(
            "No prebuilt bindings for target os: {}. Try use `gen` feature.",
            target_os
        );
    }

    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    if matches!(target_arch.as_str(), "x86" | "x86_64" | "arm" | "aarch64") {
        println!("cargo:rustc-env=LIBINPUT_TARGET_ARCH={}", target_arch);
    } else {
        panic!(
            "No prebuilt bindings for target arch: {}. Try use `gen` feature.",
            target_arch
        );
    }

    let version = lib_version();
    println!(
        "cargo:rustc-env=LIBINPUT_VERSION_STR={}_{}",
        version.0, version.1
    );
}

#[cfg(feature = "gen")]
fn main() {
    use std::path::Path;

    let version = lib_version();
    let header = Path::new("include").join(format!(
        "libinput.{}.{}.{}.h",
        version.0, version.1, version.2
    ));

    // Setup bindings builder
    let generated = bindgen::builder()
        .header(header.display().to_string())
        .ctypes_prefix("::libc")
        .whitelist_type(r"^libinput_.*$")
        .whitelist_function(r"^libinput_.*$")
        .generate()
        .unwrap();

    println!("cargo:rerun-if-changed=include/");

    // Generate the bindings
    let out_dir = env::var("OUT_DIR").unwrap();
    let bind_name = "gen.rs";
    let dest_path = Path::new(&out_dir).join(bind_name);

    generated.write_to_file(dest_path).unwrap();
}
