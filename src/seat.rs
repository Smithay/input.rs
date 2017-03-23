use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, LibinputContext};

pub struct LibinputSeat<C: 'static, D: 'static, S: 'static>
{
    seat: *mut ffi::libinput_seat,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _seat_userdata_type: PhantomData<S>,
}

impl<C: 'static, D: 'static, S: 'static> FromRaw<ffi::libinput_seat> for LibinputSeat<C, D, S>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_seat) -> LibinputSeat<C, D, S>
    {
        LibinputSeat {
            seat: ffi::libinput_seat_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, S: 'static>  AsRaw<ffi::libinput_seat> for LibinputSeat<C, D, S>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_seat {
        self.seat as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_seat {
        self.seat as *mut _
    }
}

impl<C: 'static, D: 'static, S: 'static>  Userdata<S> for LibinputSeat<C, D, S>
{
    fn userdata(&self) -> Option<&S> {
        unsafe {
            (ffi::libinput_seat_get_user_data(self.seat) as *const S).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut S> {
        unsafe {
            (ffi::libinput_seat_get_user_data(self.seat) as *mut S).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<S>) -> Option<S> {
        let old = unsafe {
            let ptr = ffi::libinput_seat_get_user_data(self.seat);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut S))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_seat_set_user_data(self.seat, match (*boxed).as_mut() {
                Some(value) => value as *mut S as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, S: 'static>  Clone for LibinputSeat<C, D, S>
{
    fn clone(&self) -> LibinputSeat<C, D, S>
    {
        LibinputSeat {
            seat: unsafe { ffi::libinput_seat_ref(self.seat) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, S: 'static> Drop for LibinputSeat<C, D, S>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_seat_get_user_data(self.seat);
            if ffi::libinput_seat_unref(self.seat).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}

impl<C: 'static, D: 'static, S: 'static> LibinputSeat<C, D, S>
{
    pub fn context(&self) -> LibinputContext<C, D, S> {
        unsafe {
            LibinputContext::from_raw(ffi::libinput_seat_get_context(self.seat))
        }
    }

    pub fn physical_name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::libinput_seat_get_physical_name(self.seat)).to_str().expect("Seat physical_name is no valid utf-8") }
    }

    pub fn logical_name(&self) -> &str {
        unsafe { CStr::from_ptr(ffi::libinput_seat_get_logical_name(self.seat)).to_str().expect("Seat logical_name is no valid utf-8") }
    }
}
