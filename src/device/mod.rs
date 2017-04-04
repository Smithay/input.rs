use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

mod capabilities;
mod group;
mod led;

pub use self::capabilities::*;
pub use self::group::*;
pub use self::led::*;

use ::{ffi, FromRaw, AsRaw, Userdata, LibinputContext, LibinputSeat, TabletPadModeGroup};

pub struct LibinputDevice<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>
{
    device: *mut ffi::libinput_device,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  FromRaw<ffi::libinput_device> for LibinputDevice<C, D, G, S, T, M>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_device) -> LibinputDevice<C, D, G, S, T, M>
    {
        LibinputDevice {
            device: ffi::libinput_device_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  AsRaw<ffi::libinput_device> for LibinputDevice<C, D, G, S, T, M>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_device {
        self.device as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_device {
        self.device as *mut _
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Userdata<D> for LibinputDevice<C, D, G, S, T, M>
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Clone for LibinputDevice<C, D, G, S, T, M>
{
    fn clone(&self) -> LibinputDevice<C, D, G, S, T, M>
    {
        LibinputDevice {
            device: unsafe { ffi::libinput_device_ref(self.device) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for LibinputDevice<C, D, G, S, T, M>
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> LibinputDevice<C, D, G, S, T, M>
{
    pub fn context(&self) -> LibinputContext<C, D, G, S, T, M>
    {
        unsafe {
            LibinputContext::from_raw(ffi::libinput_device_get_context(self.device))
        }
    }

    pub fn device_group(&self) -> LibinputDeviceGroup<C, D, G, S, T, M>
    {
        unsafe {
            LibinputDeviceGroup::from_raw(ffi::libinput_device_get_device_group(self.device))
        }
    }

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

    pub fn seat(&self) -> LibinputSeat<C, D, G, S, T, M>
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

    pub fn led_update(&mut self, leds: LibinputLed) {
        unsafe {
            ffi::libinput_device_led_update(self.device, leds.bits())
        }
    }

    pub fn has_capability(&self, cap: LibinputDeviceCapability) -> bool
    {
        match unsafe { ffi::libinput_device_has_capability(self.device, cap.into()) } {
            0 => false,
            _ => true,
        }
    }

    pub fn get_size(&self) -> Option<(f64, f64)>
    {
        let mut width = 0.0;
        let mut height = 0.0;

        match unsafe { ffi::libinput_device_get_size(self.device, &mut width as *mut _, &mut height as *mut _) } {
            0 => Some((width, height)),
            _ => None,
        }
    }

    pub fn pointer_has_button(&self, button: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_pointer_has_button(self.device, button) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    pub fn keyboard_has_key(&self, key: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_keyboard_has_key(self.device, key) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    pub fn tablet_pad_number_of_buttons(&self) -> i32
    {
        unsafe { ffi::libinput_device_tablet_pad_get_num_buttons(self.device) }
    }

    pub fn tablet_pad_number_of_rings(&self) -> i32
    {
        unsafe { ffi::libinput_device_tablet_pad_get_num_rings(self.device) }
    }

    pub fn tablet_pad_number_of_strips(&self) -> i32
    {
        unsafe { ffi::libinput_device_tablet_pad_get_num_strips(self.device) }
    }

    pub fn tablet_pad_number_of_mode_groups(&self) -> i32
    {
        unsafe { ffi::libinput_device_tablet_pad_get_num_mode_groups(self.device) }
    }

    pub fn tablet_pad_get_mode_group(&self, index: u32) -> Option<TabletPadModeGroup<C, D, G, S, T, M>> {
        let ptr = unsafe { ffi::libinput_device_tablet_pad_get_mode_group(self.device, index) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { TabletPadModeGroup::from_raw(ptr) })
        }
    }
}
