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

    #[cfg(feature = "update_bindings")]
    {
        use std::{fs, io::Write};

        let bind_file = Path::new(&out_dir).join(bind_name);
        let dest_dir = Path::new("src")
            .join("platforms")
            .join(env::var("CARGO_CFG_TARGET_OS").unwrap())
            .join(env::var("CARGO_CFG_TARGET_ARCH").unwrap());
        let dest_file = dest_dir.join(format!("gen_{}_{}.rs", version.0, version.1));

        fs::create_dir_all(&dest_dir).unwrap();
        fs::copy(&bind_file, &dest_file).unwrap();

        if let Ok(github_env) = env::var("GITHUB_ENV") {
            let mut env_file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(github_env)
                .unwrap();
            writeln!(env_file, "INPUT_SYS_BINDINGS_FILE={}", dest_file.display()).unwrap();
        }
    }
}
