use std::ffi::CString;
use std::io::{Result as IoResult, Error as IoError};
use std::os::unix::io::RawFd;
use std::{mem, ptr};
use std::iter::Iterator;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Device, Event};

pub type LibinputInterface = ffi::libinput_interface;

ffi_ref_struct!(Libinput, ffi::libinput, C, ffi::libinput_ref, ffi::libinput_unref, ffi::libinput_get_user_data, ffi::libinput_set_user_data);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Iterator for Libinput<C, D, G, S, T, M> {
    type Item = Event<C, D, G, S, T, M>;
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = unsafe { ffi::libinput_get_event(self.as_raw_mut()) };
        if ptr.is_null() {
            None
        } else {
            unsafe { Some(Event::from_raw(ptr)) }
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Libinput<C, D, G, S, T, M>
{
    pub fn new_from_udev(interface: LibinputInterface, userdata: Option<C>, udev_context: *mut libc::c_void) -> Libinput<C, D, G, S, T, M> {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: unsafe {
                ffi::libinput_udev_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut C as *mut libc::c_void,
                    None => ptr::null_mut(),
                }, udev_context as *mut _)
            },
            _userdata_type: ::std::marker::PhantomData,
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn new_from_path(interface: LibinputInterface, userdata: Option<C>) -> Libinput<C, D, G, S, T, M> {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = Libinput {
            ffi: unsafe {
                ffi::libinput_path_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut C as *mut libc::c_void,
                    None => ptr::null_mut(),
                })
            },
            _userdata_type: ::std::marker::PhantomData,
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn path_add_device(&mut self, path: &str) -> Device<C, D, G, S, T, M>
    {
        let path = CString::new(path).expect("Device Path contained a null-byte");
        unsafe {
            Device::from_raw(ffi::libinput_path_add_device(self.as_raw_mut(), path.as_ptr()))
        }
    }

    pub fn path_remove_device(&mut self, device: Device<C, D, G, S, T, M>)
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
