use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Libinput};

ffi_ref_struct!(Seat, ffi::libinput_seat, S, ffi::libinput_seat_ref, ffi::libinput_seat_unref, ffi::libinput_seat_get_user_data, ffi::libinput_seat_set_user_data);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Seat<C, D, G, S, T, M>
{
    pub fn context(&self) -> Libinput<C, D, G, S, T, M> {
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
