//!
//! # Libinput bindings for rust
//!
//! These bindings closely follow libinput's concepts and it's original API.
//! Please refer to the [libinput documentation](https://wayland.freedesktop.org/libinput/doc/latest/)
//! to understand the general structure and concepts.
//!
//! ## Differences to the C-Library:
//!
//! - Refcounting does not need to be done manually. Just call `clone` when you need an additional reference.
//! - Logging cannot be customized the same way as in libinput. Instead the [`log` crate](https://github.com/rust-lang-nursery/log) is used for maximum compatibility with the rust ecosystem.
//!
//! ## Getting started
//!
//! To get started check out the [`Libinput` struct](./struct.Libinput.html).
//!

extern crate input_sys;
extern crate libc;

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
    /// If you don't care and don't use userdata using `()` or any incompatible type is safe.
    ///
    /// ## Unsafety
    ///
    /// If the pointer is pointing to a different struct, invalid memory or `NULL` the returned
    /// struct may panic on use or cause other undefined behavior.
    ///
    unsafe fn from_raw(*mut T) -> Self;
}

/// Trait to deal with userdata attached to this struct.
pub trait Userdata<T> {
    /// Receive a raw pointer to the attached userdata.
    ///
    /// ## Unsafety
    ///
    /// Receiving userdata is unsafe as multiple references to the same underlying libinput struct
    /// may exist. As such any reference may become invalid if changed using `set_userdata` or be
    /// shared mutably.
    ///
    /// Additionally the pointer might be unset and point to `NULL`.
    ///
    unsafe fn userdata(&self) -> *mut T;
    /// Set userdata and receive the current userdata
    ///
    /// Sets new userdata or nothing in place of any existing on and returns the currently set
    /// userdata if one exists.
    ///
    /// ## Unsafety
    ///
    /// Multiple references to the same underlying libinput struct may exist. This means using this
    /// function might result in shared mutable access, which is unsafe.
    ///
    /// Additionally through `self.userdata()` more references to the returned userdata might exist.
    unsafe fn set_userdata(&mut self, new: Option<T>) -> Option<T>;
}

macro_rules! ffi_struct {
    ($struct_name:ident, $ffi_name:path) => (
        #[derive(Eq)]
        pub struct $struct_name<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>
        {
            ffi: *mut $ffi_name,
            _userdata_type: ::std::marker::PhantomData<(C, D, G, S, T, M)>,
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> ::std::fmt::Debug for $struct_name<C, D, G, S, T, M> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "$struct_name @{:p}", self.as_raw())
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<$ffi_name> for $struct_name<C, D, G, S, T, M>
        {
            unsafe fn from_raw(ffi: *mut $ffi_name) -> Self {
                $struct_name {
                    ffi: ffi,
                    _userdata_type: ::std::marker::PhantomData,
                }
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<$ffi_name> for $struct_name<C, D, G, S, T, M>
        {
            fn as_raw(&self) -> *const $ffi_name {
                self.ffi as *const _
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Clone for $struct_name<C, D, G, S, T, M> {
            fn clone(&self) -> Self {
                unsafe { $struct_name::from_raw(self.as_raw_mut()) }
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PartialEq for $struct_name<C, D, G, S, T, M> {
            fn eq(&self, other: &Self) -> bool {
                self.as_raw() == other.as_raw()
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> ::std::hash::Hash for $struct_name<C, D, G, S, T, M> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.as_raw().hash(state);
            }
        }
    )
}

macro_rules! ffi_ref_struct {
    ($struct_name:ident, $ffi_name:path, $userdata:tt, $ref_fn:path, $unref_fn:path, $get_userdata_fn:path, $set_userdata_fn:path) => (
        #[derive(Eq)]
        pub struct $struct_name<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>
        {
            ffi: *mut $ffi_name,
            _userdata_type: ::std::marker::PhantomData<(C, D, G, S, T, M)>,
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> ::std::fmt::Debug for $struct_name<C, D, G, S, T, M> {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                write!(f, "$struct_name @{:p}", self.as_raw())
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<$ffi_name> for $struct_name<C, D, G, S, T, M>
        {
            unsafe fn from_raw(ffi: *mut $ffi_name) -> Self {
                $struct_name {
                    ffi: $ref_fn(ffi),
                    _userdata_type: ::std::marker::PhantomData,
                }
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<$ffi_name> for $struct_name<C, D, G, S, T, M>
        {
            fn as_raw(&self) -> *const $ffi_name {
                self.ffi as *const _
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Userdata<$userdata> for $struct_name<C, D, G, S, T, M> {
            unsafe fn userdata(&self) -> *mut $userdata {
                $get_userdata_fn(self.as_raw_mut()) as *mut $userdata
            }

            unsafe fn set_userdata(&mut self, userdata: Option<$userdata>) -> Option<$userdata> {
                let old = unsafe {
                    let ptr = $get_userdata_fn(self.as_raw_mut());
                    if !ptr.is_null() {
                        Some(Box::from_raw(ptr as *mut $userdata))
                    } else {
                        None
                    }
                };
                let mut boxed = Box::new(userdata);
                unsafe {
                    $set_userdata_fn(self.as_raw_mut(), match (*boxed).as_mut() {
                        Some(value) => value as *mut $userdata as *mut libc::c_void,
                        None => ptr::null_mut(),
                    });
                }
                mem::forget(boxed);
                old.map(|x| *x)
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Clone for $struct_name<C, D, G, S, T, M> {
            fn clone(&self) -> Self {
                unsafe { $struct_name::from_raw(self.as_raw_mut()) }
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for $struct_name<C, D, G, S, T, M>
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

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PartialEq for $struct_name<C, D, G, S, T, M> {
            fn eq(&self, other: &Self) -> bool {
                self.as_raw() == other.as_raw()
            }
        }

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> ::std::hash::Hash for $struct_name<C, D, G, S, T, M> {
            fn hash<H: ::std::hash::Hasher>(&self, state: &mut H) {
                self.as_raw().hash(state);
            }
        }
    )
}

macro_rules! ffi_func {
    ($name:ident, $ffi_fn:path, bool) => (
        fn $name(&self) -> bool {
            unsafe { $ffi_fn(self.as_raw_mut()) != 0 }
        }
    );
    (pub $name:ident, $ffi_fn:path, bool) => (
        pub fn $name(&self) -> bool {
            unsafe { $ffi_fn(self.as_raw_mut()) != 0 }
        }
    );
    ($name:ident, $ffi_fn:path, $return_type:ty) => (
        fn $name(&self) -> $return_type {
            unsafe { $ffi_fn(self.as_raw_mut()) as $return_type }
        }
    );
    (pub $name:ident, $ffi_fn:path, $return_type:ty) => (
        pub fn $name(&self) -> $return_type {
            unsafe { $ffi_fn(self.as_raw_mut()) as $return_type }
        }
    );
}

mod context;
mod device;
mod event;
mod seat;

pub use context::*;
pub use device::*;
pub use event::*;
pub use seat::*;
