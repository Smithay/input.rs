extern crate input_sys;
extern crate libc;
#[macro_use]
extern crate bitflags;

pub mod ffi {
    pub use input_sys::*;
}

pub trait AsRaw<T> {
    fn as_raw(&self) -> *const T;
    #[doc(hidden)]
    fn as_raw_mut(&self) -> *mut T {
        self.as_raw() as *mut _
    }
}

pub trait FromRaw<T> {
    unsafe fn from_raw(*mut T) -> Self;
}

pub trait Userdata<T> {
    unsafe fn userdata(&self) -> *mut T;
    fn set_userdata(&mut self, new: Option<T>) -> Option<T>;
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
                $struct_name::from_raw(self.as_raw_mut())
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

            fn set_userdata(&mut self, userdata: Option<$userdata>) -> Option<$userdata> {
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
                $struct_name::from_raw(self.as_raw_mut())
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
