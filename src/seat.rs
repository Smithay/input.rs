use std::ffi::CStr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Libinput};

ffi_ref_struct!(struct Seat, ffi::libinput_seat, ffi::libinput_seat_ref, ffi::libinput_seat_unref, ffi::libinput_seat_get_user_data, ffi::libinput_seat_set_user_data);

impl Seat
{
    pub fn context(&self) -> Libinput {
        unsafe {
            Libinput::from_raw(ffi::libinput_seat_get_context(self.as_raw_mut()))
        }
    }

    pub fn physical_name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::libinput_seat_get_physical_name(self.as_raw_mut())).to_str().expect("Seat physical_name is no valid utf-8") }
    }

    pub fn logical_name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::libinput_seat_get_logical_name(self.as_raw_mut())).to_str().expect("Seat logical_name is no valid utf-8") }
    }
}
