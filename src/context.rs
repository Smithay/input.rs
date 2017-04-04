use std::ffi::CString;
use std::io::{Result as IoResult, Error as IoError};
use std::os::unix::io::RawFd;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, LibinputDevice};

pub type LibinputInterface = ffi::libinput_interface;

pub struct LibinputContext<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    context: *mut ffi::libinput,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput> for LibinputContext<C, D, G, S, T, M>
{
    unsafe fn from_raw(raw: *mut ffi::libinput) -> LibinputContext<C, D, G, S, T, M>
    {
        LibinputContext {
            context: ffi::libinput_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput> for LibinputContext<C, D, G, S, T, M>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput {
        self.context as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput {
        self.context
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Userdata<C> for LibinputContext<C, D, G, S, T, M>
{
    fn userdata(&self) -> Option<&C> {
        unsafe {
            (ffi::libinput_get_user_data(self.context) as *const C).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut C> {
        unsafe {
            (ffi::libinput_get_user_data(self.context) as *mut C).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<C>) -> Option<C> {
        let old = unsafe {
            let ptr = ffi::libinput_get_user_data(self.context);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut C))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_set_user_data(self.context, match (*boxed).as_mut() {
                Some(value) => value as *mut C as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Clone for LibinputContext<C, D, G, S, T, M>
{
    fn clone(&self) -> LibinputContext<C, D, G, S, T, M>
    {
        LibinputContext {
            context: unsafe { ffi::libinput_ref(self.context) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for LibinputContext<C, D, G, S, T, M>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_get_user_data(self.context);
            if ffi::libinput_unref(self.context).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> LibinputContext<C, D, G, S, T, M>
{
    pub fn new_from_udev(interface: LibinputInterface, userdata: Option<C>, udev_context: *mut libc::c_void) -> LibinputContext<C, D, G, S, T, M> {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = LibinputContext {
            context: unsafe {
                ffi::libinput_udev_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut C as *mut libc::c_void,
                    None => ptr::null_mut(),
                }, udev_context as *mut _)
            },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn new_from_path(interface: LibinputInterface, userdata: Option<C>) -> LibinputContext<C, D, G, S, T, M> {
        let boxed_interface = Box::new(interface);
        let mut boxed_userdata = Box::new(userdata);

        let context = LibinputContext {
            context: unsafe {
                ffi::libinput_path_create_context(&*boxed_interface as *const _, match (*boxed_userdata).as_mut() {
                    Some(value) => value as *mut C as *mut libc::c_void,
                    None => ptr::null_mut(),
                })
            },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        };

        mem::forget(boxed_interface);
        mem::forget(boxed_userdata);

        context
    }

    pub fn path_add_device(&mut self, path: &str) -> LibinputDevice<C, D, G, S, T, M>
    {
        let path = CString::new(path).expect("Device Path contained a null-byte");
        unsafe {
            LibinputDevice::from_raw(ffi::libinput_path_add_device(self.as_raw_mut(), path.as_ptr()))
        }
    }

    pub fn path_remove_device(&mut self, mut device: LibinputDevice<C, D, G, S, T, M>)
    {
        unsafe {
            ffi::libinput_path_remove_device(device.as_raw_mut())
        }
    }

    pub fn udev_assign_seat(&mut self, seat_id: &str) -> Result<(), ()> {
        let id = CString::new(seat_id).expect("Seat Id contained a null-byte");
        unsafe {
            match ffi::libinput_udev_assign_seat(self.context, id.as_ptr()) {
                0 => Ok(()),
                -1 => Err(()),
                _ => unreachable!(),
            }
        }
    }

    pub fn suspend(&mut self) {
        unsafe {
            ffi::libinput_suspend(self.as_raw_mut())
        }
    }

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
