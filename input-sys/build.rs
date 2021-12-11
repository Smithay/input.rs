#[cfg(feature = "gen")]
extern crate bindgen;

use std::env;

const LIB_VERSIONS: &[(u8, u8, u8)] = &[(1, 19, 0), (1, 15, 0), (1, 14, 0), (1, 11, 0), (1, 9, 0)];

fn lib_versions() -> impl Iterator<Item = &'static (u8, u8, u8)> {
    LIB_VERSIONS
        .iter()
        .filter(|version| {
            env::var(format!(
                "CARGO_FEATURE_LIBINPUT_{}_{}",
                version.0, version.1
            ))
            .is_ok()
        })
        .chain(Some(&LIB_VERSIONS[LIB_VERSIONS.len() - 1]))
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

    let version = lib_versions().next().unwrap();
    println!(
        "cargo:rustc-env=LIBINPUT_VERSION_STR={}_{}",
        version.0, version.1
    );
}

#[cfg(feature = "gen")]
fn main() {
    use std::path::Path;

    let version = lib_versions().next().unwrap();
    println!(
        "cargo:rustc-env=LIBINPUT_VERSION_STR={}_{}",
        version.0, version.1
    );

    #[cfg(feature = "update_bindings")]
    let dest_dir = Path::new("src")
        .join("platforms")
        .join(env::var("CARGO_CFG_TARGET_OS").unwrap())
        .join(env::var("CARGO_CFG_TARGET_ARCH").unwrap());

    for version in lib_versions() {
        let header = Path::new("include").join(format!(
            "libinput.{}.{}.{}.h",
            version.0, version.1, version.2
        ));

        // Setup bindings builder
        let generated = bindgen::builder()
            .clang_arg(match cfg!(target_os = "freebsd") {
                true => "-I/usr/local/include",
                false => "",
            })
            .header(header.display().to_string())
            .ctypes_prefix("::libc")
            .whitelist_type(r"^libinput_.*$")
            .whitelist_function(r"^libinput_.*$")
            .generate()
            .unwrap();

        println!("cargo:rerun-if-changed=include/");

        // Generate the bindings
        let out_dir = env::var("OUT_DIR").unwrap();
        let bind_name = format!("gen_{}_{}.rs", version.0, version.1);
        let dest_path = Path::new(&out_dir).join(&bind_name);

        generated.write_to_file(dest_path).unwrap();

        #[cfg(feature = "update_bindings")]
        {
            use std::fs;

            let bind_file = Path::new(&out_dir).join(&bind_name);
            let dest_file = dest_dir.join(&bind_name);

            fs::create_dir_all(&dest_dir).unwrap();
            fs::copy(&bind_file, &dest_file).unwrap();
        }
    }

    #[cfg(feature = "update_bindings")]
    {
        use std::{fs, io::Write};

        if let Ok(github_env) = env::var("GITHUB_ENV") {
            let mut env_file = fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(github_env)
                .unwrap();
            writeln!(env_file, "INPUT_SYS_BINDINGS_FILE={}", dest_dir.display()).unwrap();
        }
    }
}
