use ::{ffi, FromRaw, AsRaw, Userdata};

use libc;

ffi_ref_struct!(struct TabletPadModeGroup, ffi::libinput_tablet_pad_mode_group, ffi::libinput_tablet_pad_mode_group_ref, ffi::libinput_tablet_pad_mode_group_unref, ffi::libinput_tablet_pad_mode_group_get_user_data, ffi::libinput_tablet_pad_mode_group_set_user_data);

impl TabletPadModeGroup {
    pub fn button_is_toggle(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_button_is_toggle(self.as_raw_mut(), button) != 0 }
    }

    ffi_func!(pub fn index, ffi::libinput_tablet_pad_mode_group_get_index, u32);
    ffi_func!(pub fn mode, ffi::libinput_tablet_pad_mode_group_get_mode, u32);
    ffi_func!(pub fn number_of_modes, ffi::libinput_tablet_pad_mode_group_get_num_modes, u32);

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
