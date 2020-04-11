#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;

#[cfg(feature = "gen")]
include!(concat!(env!("OUT_DIR"), "/gen.rs"));

#[cfg(all(not(feature = "gen"), feature="libinput_1_15"))]
include!("gen_1_15.rs");
#[cfg(all(not(feature = "gen"), feature="libinput_1_14", not(feature="libinput_1_15")))]
include!("gen_1_14.rs");
#[cfg(all(not(feature = "gen"), feature="libinput_1_11", not(any(feature="libinput_1_14", feature="libinput_1_15"))))]
include!("gen_1_11.rs");
#[cfg(all(not(feature = "gen"), not(any(feature="libinput_1_11", feature="libinput_1_14", feature="libinput_1_15"))))]
include!("gen_1_9.rs");

#[link(name = "input")]
extern {}
