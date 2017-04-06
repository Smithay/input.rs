use std::ffi::{CStr, CString};
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Libinput, Seat, TabletPadModeGroup};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability
{
    Keyboard,
    Pointer,
    Touch,
    TabletTool,
    TabletPad,
    Gesture,
    Switch,
}

bitflags! {
    pub flags Led: u32 {
        const NUM_LOCK = ffi::libinput_led_LIBINPUT_LED_NUM_LOCK,
        const CAPS_LOCK = ffi::libinput_led_LIBINPUT_LED_CAPS_LOCK,
        const SCROLL_LOCK = ffi::libinput_led_LIBINPUT_LED_SCROLL_LOCK,
    }
}

ffi_ref_struct!(DeviceGroup, ffi::libinput_device_group, G, ffi::libinput_device_group_ref, ffi::libinput_device_group_unref, ffi::libinput_device_group_get_user_data, ffi::libinput_device_group_set_user_data);

ffi_ref_struct!(Device, ffi::libinput_device, D, ffi::libinput_device_ref, ffi::libinput_device_unref, ffi::libinput_device_get_user_data, ffi::libinput_device_set_user_data);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Device<C, D, G, S, T, M>
{
    pub fn context(&self) -> Libinput<C, D, G, S, T, M>
    {
        unsafe {
            Libinput::from_raw(ffi::libinput_device_get_context(self.as_raw_mut()))
        }
    }

    pub fn device_group(&self) -> DeviceGroup<C, D, G, S, T, M>
    {
        unsafe {
            DeviceGroup::from_raw(ffi::libinput_device_get_device_group(self.as_raw_mut()))
        }
    }

    pub fn sysname(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_sysname(self.as_raw_mut()) ).to_str().expect("Device sysname is no valid utf8")
        }
    }

    pub fn name(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_name(self.as_raw_mut()) ).to_str().expect("Device name is no valid utf8")
        }
    }

    pub fn output_name(&self) -> Option<&str>
    {
        unsafe {
            let ptr = ffi::libinput_device_get_output_name(self.as_raw_mut());
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr).to_str().expect("Device output_name is no valid utf8"))
            } else {
                None
            }
        }
    }

    ffi_func!(pub id_product, ffi::libinput_device_get_id_product, u32);
    ffi_func!(pub id_vendor, ffi::libinput_device_get_id_product, u32);

    pub fn seat(&self) -> Seat<C, D, G, S, T, M>
    {
        unsafe {
            Seat::from_raw(ffi::libinput_device_get_seat(self.as_raw_mut()))
        }
    }

    pub fn set_seat_logical_name(&mut self, name: &str) -> Result<(), ()>
    {
        let name = CString::new(name).expect("New logical_seat name contained a null-byte");
        unsafe {
            if ffi::libinput_device_set_seat_logical_name(self.as_raw_mut(), name.as_ptr()) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    ffi_func!(pub udev_device, ffi::libinput_device_get_udev_device, *mut libc::c_void);

    pub fn led_update(&mut self, leds: Led) {
        unsafe {
            ffi::libinput_device_led_update(self.as_raw_mut(), leds.bits())
        }
    }

    pub fn has_capability(&self, cap: DeviceCapability) -> bool
    {
        unsafe { ffi::libinput_device_has_capability(self.as_raw_mut(), match cap {
            DeviceCapability::Keyboard => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_KEYBOARD,
            DeviceCapability::Pointer => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_POINTER,
            DeviceCapability::Touch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TOUCH,
            DeviceCapability::TabletTool => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_TOOL,
            DeviceCapability::TabletPad => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_PAD,
            DeviceCapability::Gesture => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_GESTURE,
            DeviceCapability::Switch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_SWITCH,
        }) != 0 }
    }

    pub fn get_size(&self) -> Option<(f64, f64)>
    {
        let mut width = 0.0;
        let mut height = 0.0;

        match unsafe { ffi::libinput_device_get_size(self.as_raw_mut(), &mut width as *mut _, &mut height as *mut _) } {
            0 => Some((width, height)),
            _ => None,
        }
    }

    pub fn pointer_has_button(&self, button: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_pointer_has_button(self.as_raw_mut(), button) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    pub fn keyboard_has_key(&self, key: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_keyboard_has_key(self.as_raw_mut(), key) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    ffi_func!(pub tablet_pad_number_of_buttons, ffi::libinput_device_tablet_pad_get_num_buttons, i32);
    ffi_func!(pub tablet_pad_number_of_rings, ffi::libinput_device_tablet_pad_get_num_rings, i32);
    ffi_func!(pub tablet_pad_number_of_strips, ffi::libinput_device_tablet_pad_get_num_strips, i32);
    ffi_func!(pub tablet_pad_number_of_mode_groups, ffi::libinput_device_tablet_pad_get_num_mode_groups, i32);

    pub fn tablet_pad_get_mode_group(&self, index: u32) -> Option<TabletPadModeGroup<C, D, G, S, T, M>> {
        let ptr = unsafe { ffi::libinput_device_tablet_pad_get_mode_group(self.as_raw_mut(), index) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { TabletPadModeGroup::from_raw(ptr) })
        }
    }
}
