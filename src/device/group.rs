use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata};

pub struct LibinputDeviceGroup<C: 'static, D: 'static, G: 'static, S: 'static>
{
    device_group: *mut ffi::libinput_device_group,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static>  FromRaw<ffi::libinput_device_group> for LibinputDeviceGroup<C, D, G, S>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_device_group) -> LibinputDeviceGroup<C, D, G, S>
    {
        LibinputDeviceGroup {
            device_group: ffi::libinput_device_group_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static>  AsRaw<ffi::libinput_device_group> for LibinputDeviceGroup<C, D, G, S>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_device_group {
        self.device_group as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_device_group {
        self.device_group as *mut _
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static>  Userdata<D> for LibinputDeviceGroup<C, D, G, S>
{
    fn userdata(&self) -> Option<&D> {
        unsafe {
            (ffi::libinput_device_group_get_user_data(self.device_group) as *const D).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut D> {
        unsafe {
            (ffi::libinput_device_group_get_user_data(self.device_group) as *mut D).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<D>) -> Option<D> {
        let old = unsafe {
            let ptr = ffi::libinput_device_group_get_user_data(self.device_group);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut D))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_device_group_set_user_data(self.device_group, match (*boxed).as_mut() {
                Some(value) => value as *mut D as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static>  Clone for LibinputDeviceGroup<C, D, G, S>
{
    fn clone(&self) -> LibinputDeviceGroup<C, D, G, S>
    {
        LibinputDeviceGroup {
            device_group: unsafe { ffi::libinput_device_group_ref(self.device_group) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static> Drop for LibinputDeviceGroup<C, D, G, S>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_device_group_get_user_data(self.device_group);
            if ffi::libinput_device_group_unref(self.device_group).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}
