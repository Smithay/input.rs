#![allow(non_camel_case_types, non_upper_case_globals)]

#[cfg(feature = "use_bindgen")]
include!(concat!(
    env!("OUT_DIR"),
    "/gen_",
    env!("LIBINPUT_VERSION_STR"),
    ".rs"
));

#[cfg(not(feature = "use_bindgen"))]
include!(concat!(
    "bindings/gen_",
    env!("LIBINPUT_VERSION_STR"),
    ".rs"
));

#[link(name = "input")]
extern "C" {}
