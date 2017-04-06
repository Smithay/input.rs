use ::{ffi, FromRaw, AsRaw, Userdata};

use libc;

use std::{mem, ptr};

ffi_ref_struct!(TabletPadModeGroup, ffi::libinput_tablet_pad_mode_group, M, ffi::libinput_tablet_pad_mode_group_ref, ffi::libinput_tablet_pad_mode_group_unref, ffi::libinput_tablet_pad_mode_group_get_user_data, ffi::libinput_tablet_pad_mode_group_set_user_data);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadModeGroup<C, D, G, S, T, M> {
    pub fn button_is_toggle(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_button_is_toggle(self.as_raw_mut(), button) != 0 }
    }

    ffi_func!(pub index, ffi::libinput_tablet_pad_mode_group_get_index, u32);
    ffi_func!(pub mode, ffi::libinput_tablet_pad_mode_group_get_mode, u32);
    ffi_func!(pub number_of_modes, ffi::libinput_tablet_pad_mode_group_get_num_modes, u32);

    pub fn has_button(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_button(self.as_raw_mut(), button) != 0 }
    }

    pub fn has_ring(&self, ring: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_ring(self.as_raw_mut(), ring) != 0 }
    }

    pub fn has_strip(&self, strip: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_strip(self.as_raw_mut(), strip) != 0 }
    }
}
