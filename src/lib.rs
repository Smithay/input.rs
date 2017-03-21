extern crate input_sys;
extern crate libc;

pub mod ffi {
    pub use input_sys::*;
}

use libc::c_void;

use std::io::{Result as IoResult, Error as IoError};
use std::os::unix::io::RawFd;

use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

pub trait AsRaw<T> {
    unsafe fn as_raw(&self) -> *const T;
    unsafe fn as_raw_mut(&mut self) -> *mut T;
}

pub trait LibinputInterface<U: 'static> {
    fn open_restricted(&mut self, path: &str, flags: i32, userdata: &mut U) -> IoResult<RawFd>;
    fn close_restricted(&mut self, fd: RawFd, userdata: &mut U);
}

pub trait LibinputContext<U: 'static>: AsRaw<ffi::libinput> {
    fn userdata(&self) -> &U;
    fn userdata_mut(&mut self) -> &mut U;
    fn set_userdata(&mut self, userdata: U) -> U;

    fn suspend(&mut self) {
        unsafe {
            ffi::libinput_suspend(self.as_raw_mut())
        }
    }

    fn resume(&mut self) -> Result<(), ()> {
        unsafe {
            match ffi::libinput_resume(self.as_raw_mut()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    fn dispatch(&mut self) -> IoResult<()> {
        unsafe {
            match ffi::libinput_dispatch(self.as_raw_mut()) {
                0 => Ok(()),
                x if x < 0 => Err(IoError::from_raw_os_error(x)),
                _ => unreachable!(),
            }
        }
    }

    unsafe fn fd(&mut self) -> RawFd {
        ffi::libinput_get_fd(self.as_raw_mut())
    }
}

pub struct LibinputContextUdev<U: 'static, I: LibinputInterface<U> + 'static> {
    context: *mut ffi::libinput,
    interface: I,
    userdata: U,
}

unsafe extern "C" fn open_restricted_callback_udev<U: 'static, I: LibinputInterface<U> + 'static>(path: *const libc::c_char, flags: libc::c_int, userdata: *mut libc::c_void) -> libc::c_int {
    let context = userdata as *mut LibinputContextUdev<U, I>;
    match (*context).interface.open_restricted(CStr::from_ptr(path).to_str().expect("Device Path was no valid UTF-8"), flags, &mut (*context).userdata) {
        Ok(fd) => fd,
        Err(error) => match error.raw_os_error() {
            Some(errno) => -errno,
            None => -5,
        }
    }
}

unsafe extern "C" fn close_restricted_callback_udev<U: 'static, I: LibinputInterface<U> + 'static>(fd: libc::c_int, userdata: *mut libc::c_void) {
    let context = userdata as *mut LibinputContextUdev<U, I>;
    (*context).interface.close_restricted(fd, &mut (*context).userdata)
}

impl<U: 'static, I: LibinputInterface<U> + 'static> LibinputContextUdev<U, I> {
    pub unsafe fn new(interface: I, userdata: U, udev_context: *mut c_void) -> LibinputContextUdev<U, I> {
        let mut new = LibinputContextUdev {
            context: ptr::null_mut(),
            interface: interface,
            userdata: userdata,
        };

        let interface = Box::new(ffi::libinput_interface {
            open_restricted: Some(open_restricted_callback_udev::<U, I>),
            close_restricted: Some(close_restricted_callback_udev::<U, I>),
        });

        let context = ffi::libinput_udev_create_context(&*interface as *const _, &mut new as *mut LibinputContextUdev<U, I> as *mut c_void, udev_context as *mut _);
        new.context = context;

        mem::forget(interface);

        new
    }

    pub fn assign_seat(&mut self, seat_id: &str) -> Result<(), ()> {
        let id = CString::new(seat_id).expect("Seat Id contained a nul-byte");
        unsafe {
            match ffi::libinput_udev_assign_seat(self.context, id.as_ptr()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }
}

impl<U: 'static, I: LibinputInterface<U> + 'static> AsRaw<ffi::libinput> for LibinputContextUdev<U, I>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput {
        self.context as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput {
        self.context
    }
}

impl<U: 'static, I: LibinputInterface<U> + 'static> LibinputContext<U> for LibinputContextUdev<U, I>
{
    fn userdata(&self) -> &U {
        &self.userdata
    }

    fn userdata_mut(&mut self) -> &mut U {
        &mut self.userdata
    }

    fn set_userdata(&mut self, mut userdata: U) -> U {
        mem::swap(&mut userdata, &mut self.userdata);
        userdata
    }
}

impl<U: 'static, I: LibinputInterface<U> + 'static>  Drop for LibinputContextUdev<U, I>
{
    fn drop(&mut self) {
        unsafe {
            ffi::libinput_unref(self.context);
        }
    }
}

pub struct LibinputContextPath<U: 'static, I: LibinputInterface<U> + 'static> {
    context: *mut ffi::libinput,
    interface: I,
    userdata: U,
}

unsafe extern "C" fn open_restricted_callback_path<U: 'static, I: LibinputInterface<U> + 'static>(path: *const libc::c_char, flags: libc::c_int, userdata: *mut libc::c_void) -> libc::c_int {
    let context = userdata as *mut LibinputContextPath<U, I>;
    match (*context).interface.open_restricted(CStr::from_ptr(path).to_str().expect("Device Path was no valid UTF-8"), flags, &mut (*context).userdata) {
        Ok(fd) => fd,
        Err(error) => match error.raw_os_error() {
            Some(errno) => -errno,
            None => -5,
        }
    }
}

unsafe extern "C" fn close_restricted_callback_path<U: 'static, I: LibinputInterface<U> + 'static>(fd: libc::c_int, userdata: *mut libc::c_void) {
    let context = userdata as *mut LibinputContextPath<U, I>;
    (*context).interface.close_restricted(fd, &mut (*context).userdata)
}

impl<U: 'static, I: LibinputInterface<U> + 'static> LibinputContextPath<U, I> {
    pub unsafe fn new(interface: I, userdata: U) -> LibinputContextPath<U, I> {
        let mut new = LibinputContextPath {
            context: ptr::null_mut(),
            interface: interface,
            userdata: userdata,
        };

        let interface = Box::new(ffi::libinput_interface {
            open_restricted: Some(open_restricted_callback_path::<U, I>),
            close_restricted: Some(close_restricted_callback_path::<U, I>),
        });

        let context = ffi::libinput_path_create_context(&*interface as *const _, &mut new as *mut LibinputContextPath<U, I> as *mut c_void);
        new.context = context;

        mem::forget(interface);

        new
    }

    /*
    pub fn add_device(&mut self, path: &str) -> LibinputDevice {}
    pub fn remove_device(&mut self, device: LipinputDevice) {}
    */
}

impl<U: 'static, I: LibinputInterface<U> + 'static> AsRaw<ffi::libinput> for LibinputContextPath<U, I>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput {
        self.context as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput {
        self.context
    }
}

impl<U: 'static, I: LibinputInterface<U> + 'static> LibinputContext<U> for LibinputContextPath<U, I>
{
    fn userdata(&self) -> &U {
        &self.userdata
    }

    fn userdata_mut(&mut self) -> &mut U {
        &mut self.userdata
    }

    fn set_userdata(&mut self, mut userdata: U) -> U {
        mem::swap(&mut userdata, &mut self.userdata);
        userdata
    }
}

impl<U: 'static, I: LibinputInterface<U> + 'static>  Drop for LibinputContextPath<U, I>
{
    fn drop(&mut self) {
        unsafe {
            ffi::libinput_unref(self.context);
        }
    }
}
