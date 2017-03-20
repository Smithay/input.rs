extern crate bindgen;

fn main()
{
    // Setup bindings builder
    let generated = bindgen::builder()
        .header("include/libinput.1.6.3.h")
        .no_unstable_rust()
        .ctypes_prefix("libc")
        .whitelisted_type(r"^libinput_.*$")
        .whitelisted_function(r"^libinput_.*$")
        .generate().unwrap();

    println!("cargo:rustc-link-lib=dylib=input");

    // Generate the bindings
    generated.write_to_file("src/gen.rs").unwrap();
}
