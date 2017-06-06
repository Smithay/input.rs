#![allow(non_camel_case_types, non_upper_case_globals)]

extern crate libc;

#[cfg(feature = "gen")]
include!(concat!(env!("OUT_DIR"), "/gen.rs"));

#[cfg(not(feature = "gen"))]
include!("gen.rs");

#[link(name = "input")]
extern {}
