use std::ffi::CString;
use std::io::{Result as IoResult, Error as IoError};
use std::os::unix::io::RawFd;
use std::{mem, ptr};
use std::iter::Iterator;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Device, Event};

pub type LibinputInterface = ffi::libinput_interface;

/// Libinput context
///
/// Contexts can be used to track input devices and receive events from them.
/// You can use either `new_from_udev` to create a context tracking all devices on a specific seat,
/// or use `new_from_path` to track input devices manually.
///
/// Either way you then have to use `dispatch()` and `next()` (provided by the `Iterator` trait) to
/// receive events.
ffi_ref_struct!(Libinput, ffi::libinput, ffi::libinput_ref, ffi::libinput_unref, ffi::libinput_get_user_data, ffi::libinput_set_user_data);

impl Iterator for Libinput {
    type Item = Event;
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = unsafe { ffi::libinput_get_event(self.as_raw_mut()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Event::from_raw(ptr)) }
        }
    }
}

impl Libinput
{
    /// Create a new libinput context using a udev context.
    ///
    /// ## Arguments
    ///
    /// - interface - A `LibinputInterface` providing functions to open and close devices.
    /// - userdata - Optionally some userdata attached to the newly created context (see [`Userdata`](./trait.Userdata.html))
    /// - udev_context - Raw pointer to a valid udev context.
    ///
    /// ## Unsafety
    ///
    /// This function is unsafe, because there is no way to verify that `udev_context` is indeed a valid udev context or even points to valid memory.
    pub unsafe fn new_from_udev<T: 'static>(interface: LibinputInterface, userdata: Option<T>, udev_context: *mut libc::c_void) -> Libinput {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: {
                ffi::libinput_udev_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut T as *mut libc::c_void,
                    None => ptr::null_mut(),
                }, udev_context as *mut _)
            },
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn new_from_path<T: 'static>(interface: LibinputInterface, userdata: Option<T>) -> Libinput {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: unsafe {
                ffi::libinput_path_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut T as *mut libc::c_void,
                    None => ptr::null_mut(),
                })
            },
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn path_add_device(&mut self, path: &str) -> Device
    {
        let path = CString::new(path).expect("Device Path contained a null-byte");
        unsafe {
            Device::from_raw(ffi::libinput_path_add_device(self.as_raw_mut(), path.as_ptr()))
        }
    }

    pub fn path_remove_device(&mut self, device: Device)
    {
        unsafe {
            ffi::libinput_path_remove_device(device.as_raw_mut())
        }
    }

    pub fn udev_assign_seat(&mut self, seat_id: &str) -> Result<(), ()> {
        let id = CString::new(seat_id).expect("Seat Id contained a null-byte");
        unsafe {
            match ffi::libinput_udev_assign_seat(self.as_raw_mut(), id.as_ptr()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    ffi_func!(pub suspend, ffi::libinput_suspend, ());

    pub fn resume(&mut self) -> Result<(), ()> {
        unsafe {
            match ffi::libinput_resume(self.as_raw_mut()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    pub fn dispatch(&mut self) -> IoResult<()> {
        unsafe {
            match ffi::libinput_dispatch(self.as_raw_mut()) {
                0 => Ok(()),
                x if x < 0 => Err(IoError::from_raw_os_error(x)),
                _ => unreachable!(),
            }
        }
    }

    pub unsafe fn fd(&mut self) -> RawFd {
        ffi::libinput_get_fd(self.as_raw_mut())
    }
}
