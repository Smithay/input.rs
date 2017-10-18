//! # Libinput bindings for rust
//!
//! These bindings closely follow libinput's concepts and it's original API.
//! Please refer to the [libinput documentation](https://wayland.freedesktop.org/libinput/doc/latest/)
//! to understand the general structure and concepts.
//!
//! ## Differences to the C-Library:
//!
//! - Refcounting does not need to be done manually. Just call `clone` when you need an additional reference.
//! - Libinput logging cannot (currently) not be customized.
//!
//! ## Userdata handling
//!
//! Multiple types in the libinput library allow to attach a pointer of an arbitrary type, so called `userdata`.
//! Using this data is unsafe as there is no way to find out what type is stored in the libinput struct.
//! Additionally multiple references to the same libinput object may exist and userdata may be shared mutably.
//!
//! This is why using and setting userdata is an unsafe operation (except when creating an object).
//!
//! If you heavily rely on userdata, you should always stored them wrapped in a `Mutex` and use the same
//! type for every userdata access to further simplify usage.
//!
//! You need to be especially cautious when initializing libinput types from raw pointers, you obtained
//! from other libraries which may set their own userdata. If accessing their userdata make sure no shared
//! mutable access may happen and don't store something else instead, if the library does not explicitly
//! allow this.
//!
//! Generally usage of this api is error-prone and discouraged if not needed.
//!
//! ## Getting started
//!
//! To get started check out the [`Libinput` struct](./struct.Libinput.html).
//!

#![deny(missing_docs)]
#![cfg_attr(feature="cargo-clippy", deny(clippy))]
#![cfg_attr(feature="cargo-clippy", allow(doc_markdown))]

extern crate input_sys;
extern crate libc;

#[macro_use]
extern crate bitflags;

use std::{mem, ptr};

/// Unsafe c-api.
pub mod ffi {
    pub use input_sys::*;
}

/// Trait for types that allow to optain the underlying raw libinput pointer.
pub trait AsRaw<T> {
    /// Receive a raw pointer representing this type.
    fn as_raw(&self) -> *const T;
    #[doc(hidden)]
    fn as_raw_mut(&self) -> *mut T {
        self.as_raw() as *mut _
    }
}

/// Trait for types that allow to be initialized from a raw pointer
pub trait FromRaw<T> {
    /// Create a new instance of this type from a raw pointer.
    ///
    /// ## Warning
    ///
    /// If you make use of [`Userdata`](./trait.Userdata.html) make sure you use the correct types
    /// to allow receiving the set userdata. When dealing with raw pointers initialized by other
    /// libraries this must be done extra carefully to select a correct representation.
    ///
    /// If unsure using `()` is always a safe option..
    ///
    /// ## Unsafety
    ///
    /// If the pointer is pointing to a different struct, invalid memory or `NULL` the returned
    /// struct may panic on use or cause other undefined behavior.
    ///
    unsafe fn from_raw(*mut T) -> Self;
}

/// Trait to deal with userdata attached to this struct.
pub trait Userdata {
    /// Receive a reference to the attached userdata, if one exists.
    ///
    /// ## Unsafety
    ///
    /// Receiving userdata is unsafe as multiple references to the same underlying libinput struct
    /// may exist. As such any reference may become invalid if changed using `set_userdata`.
    ///
    unsafe fn userdata<T: 'static>(&self) -> Option<&T> {
        self.userdata_raw().as_ref()
    }

    /// Receive a mutable reference to the attached userdata, if one exists.
    ///
    /// ## Unsafety
    ///
    /// Receiving userdata is unsafe as multiple references to the same underlying libinput struct
    /// may exist. As such any reference may become invalid if changed using `set_userdata`.
    ///
    /// Additionally multiple mutable references may be created through the existance of multiple
    /// structs using the same libinput reference. If you have control over the userdata make sure
    /// to store `Mutex` to be on the safe side.
    ///
    unsafe fn userdata_mut<T: 'static>(&mut self) -> Option<&mut T> {
        self.userdata_raw().as_mut()
    }

    /// Set userdata and receive the currently set userdata
    ///
    /// Sets new userdata or nothing in place of any existing one and returns the currently set
    /// userdata if one exists.
    ///
    /// ## Unsafety
    ///
    /// Multiple references to the same underlying libinput struct may exist. As such this function
    /// allows for shared mutable access, which is unsafe. Also this means this function
    /// might invalidate references to the currently set userdata, once dropped.
    ///
    /// Use a `Mutex` when storing new userdata to mitiage some of these problems.
    ///
    /// ## Note
    ///
    /// Stored userdata is dropped correctly, if the last reference is hold by rust and goes out of scope normally.
    unsafe fn set_userdata<T: 'static, U: 'static>(&mut self, new: Option<T>) -> Option<U> {
        let old = {
            let ptr = self.userdata_raw();
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut U))
            } else {
                None
            }
        };
        let mut boxed = Box::new(new);
        self.set_userdata_raw(match (*boxed).as_mut() {
                                  Some(value) => value as *mut T,
                                  None => ptr::null_mut(),
                              });
        mem::forget(boxed);
        old.map(|x| *x)
    }

    #[doc(hidden)]
    unsafe fn userdata_raw<T: 'static>(&self) -> *mut T;
    #[doc(hidden)]
    unsafe fn set_userdata_raw<T: 'static>(&self, ptr: *mut T);
}

macro_rules! ffi_ref_struct {
    ($(#[$attr:meta])* struct $struct_name:ident, $ffi_name:path, $ref_fn:path, $unref_fn:path, $get_userdata_fn:path, $set_userdata_fn:path) => (
        #[derive(Eq)]
        $(#[$attr])*
        pub struct $struct_name
        {
            ffi: *mut $ffi_name,
        }

        impl ::std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "$struct_name @{:p}", self.as_raw())
            }
        }

        impl FromRaw<$ffi_name> for $struct_name
        {
            unsafe fn from_raw(ffi: *mut $ffi_name) -> Self {
                $struct_name {
                    ffi: $ref_fn(ffi),
                }
            }
        }

        impl AsRaw<$ffi_name> for $struct_name
        {
            fn as_raw(&self) -> *const $ffi_name {
                self.ffi as *const _
            }
        }

        impl Userdata for $struct_name {
            unsafe fn userdata_raw<T: 'static>(&self) -> *mut T {
                $get_userdata_fn(self.as_raw_mut()) as *mut T
            }
            unsafe fn set_userdata_raw<T: 'static>(&self, ptr: *mut T) {
                $set_userdata_fn(self.as_raw_mut(), ptr as *mut libc::c_void);
            }
        }

        impl Clone for $struct_name {
            fn clone(&self) -> Self {
                unsafe { $struct_name::from_raw(self.as_raw_mut()) }
            }
        }

        impl Drop for $struct_name
        {
            fn drop(&mut self) {
                unsafe {
                    let userdata_ref = $get_userdata_fn(self.as_raw_mut());
                    if $unref_fn(self.ffi).is_null() {
                        let _ = Box::from_raw(userdata_ref);
                    }
                }
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.as_raw() == other.as_raw()
            }
        }

        impl ::std::hash::Hash for $struct_name {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.as_raw().hash(state);
            }
        }
    )
}

macro_rules! ffi_func {
    ($(#[$attr:meta])* fn $name:ident, $ffi_fn:path, bool) => (
        $(#[$attr])*
        fn $name(&self) -> bool {
            unsafe { $ffi_fn(self.as_raw_mut()) != 0 }
        }
    );
    ($(#[$attr:meta])* pub fn $name:ident, $ffi_fn:path, bool) => (
        $(#[$attr])*
        pub fn $name(&self) -> bool {
            unsafe { $ffi_fn(self.as_raw_mut()) != 0 }
        }
    );
    ($(#[$attr:meta])* fn $name:ident, $ffi_fn:path, $return_type:ty) => (
        $(#[$attr])*
        fn $name(&self) -> $return_type {
            unsafe { $ffi_fn(self.as_raw_mut()) as $return_type }
        }
    );
    ($(#[$attr:meta])* pub fn $name:ident, $ffi_fn:path, $return_type:ty) => (
        $(#[$attr])*
        pub fn $name(&self) -> $return_type {
            unsafe { $ffi_fn(self.as_raw_mut()) as $return_type }
        }
    );
}

mod context;
mod device;
pub mod event;
mod seat;

pub use context::*;
pub use device::*;
pub use event::Event;
pub use seat::*;
