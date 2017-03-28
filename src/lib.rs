extern crate input_sys;
extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod ffi {
    pub use input_sys::*;
}

pub trait AsRaw<T> {
    unsafe fn as_raw(&self) -> *const T;
    unsafe fn as_raw_mut(&mut self) -> *mut T;
}

pub trait FromRaw<T> {
    unsafe fn from_raw(*mut T) -> Self;
}

pub trait Userdata<T> {
    fn userdata(&self) -> Option<&T>;
    fn userdata_mut(&mut self) -> Option<&mut T>;
    fn set_userdata(&mut self, new: Option<T>) -> Option<T>;
}

mod context;
mod device;
mod event;
mod seat;

pub use context::*;
pub use device::*;
pub use event::*;
pub use seat::*;
