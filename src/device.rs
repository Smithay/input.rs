use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, LibinputContext, LibinputSeat};

pub struct LibinputDevice<C: 'static, D: 'static, S: 'static>
{
    device: *mut ffi::libinput_device,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _seat_userdata_type: PhantomData<S>,
}

impl<C: 'static, D: 'static, S: 'static>  FromRaw<ffi::libinput_device> for LibinputDevice<C, D, S>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_device) -> LibinputDevice<C, D, S>
    {
        LibinputDevice {
            device: ffi::libinput_device_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, S: 'static>  AsRaw<ffi::libinput_device> for LibinputDevice<C, D, S>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_device {
        self.device as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_device {
        self.device as *mut _
    }
}

impl<C: 'static, D: 'static, S: 'static>  Userdata<D> for LibinputDevice<C, D, S>
{
    fn userdata(&self) -> Option<&D> {
        unsafe {
            (ffi::libinput_device_get_user_data(self.device) as *const D).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut D> {
        unsafe {
            (ffi::libinput_device_get_user_data(self.device) as *mut D).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<D>) -> Option<D> {
        let old = unsafe {
            let ptr = ffi::libinput_device_get_user_data(self.device);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut D))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_device_set_user_data(self.device, match (*boxed).as_mut() {
                Some(value) => value as *mut D as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, S: 'static>  Clone for LibinputDevice<C, D, S>
{
    fn clone(&self) -> LibinputDevice<C, D, S>
    {
        LibinputDevice {
            device: unsafe { ffi::libinput_device_ref(self.device) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, S: 'static> Drop for LibinputDevice<C, D, S>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_device_get_user_data(self.device);
            if ffi::libinput_device_unref(self.device).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}

impl<C: 'static, D: 'static, S: 'static> LibinputDevice<C, D, S>
{
    pub fn context(&self) -> LibinputContext<C, D, S>
    {
        unsafe {
            LibinputContext::from_raw(ffi::libinput_device_get_context(self.device))
        }
    }

    /*
    pub fn device_group(&self) -> LibinputDeviceGroup
    {
        unsafe {
            LibinputDeviceGroup::from_raw(ffi::libinput_device_get_display_group(self.device))
        }
    }
    */

    pub fn sysname(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_sysname(self.device) ).to_str().expect("Device sysname is no valid utf8")
        }
    }

    pub fn name(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_name(self.device) ).to_str().expect("Device name is no valid utf8")
        }
    }

    pub fn output_name(&self) -> Option<&str>
    {
        unsafe {
            let ptr = ffi::libinput_device_get_output_name(self.device);
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr).to_str().expect("Device output_name is no valid utf8"))
            } else {
                None
            }
        }
    }

    pub fn id_product(&self) -> u32
    {
        unsafe {
            ffi::libinput_device_get_id_product(self.device)
        }
    }

    pub fn id_vendor(&self) -> u32
    {
        unsafe {
            ffi::libinput_device_get_id_vendor(self.device)
        }
    }

    pub fn seat(&self) -> LibinputSeat<C, D, S>
    {
        unsafe {
            LibinputSeat::from_raw(ffi::libinput_device_get_seat(self.device))
        }
    }

    pub fn set_seat_logical_name(&mut self, name: &str) -> Result<(), ()>
    {
        let name = CString::new(name).expect("New logical_seat name contained a null-byte");
        unsafe {
            if ffi::libinput_device_set_seat_logical_name(self.device, name.as_ptr()) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    pub unsafe fn udev_device(&self) -> *mut libc::c_void
    {
        ffi::libinput_device_get_udev_device(self.device) as *mut _
    }
}
