use std::ffi::CStr;
use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, LibinputContext};

pub struct LibinputSeat<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>
{
    seat: *mut ffi::libinput_seat,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_seat> for LibinputSeat<C, D, G, S, T, M>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_seat) -> LibinputSeat<C, D, G, S, T, M>
    {
        LibinputSeat {
            seat: ffi::libinput_seat_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  AsRaw<ffi::libinput_seat> for LibinputSeat<C, D, G, S, T, M>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_seat {
        self.seat as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_seat {
        self.seat as *mut _
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Userdata<S> for LibinputSeat<C, D, G, S, T, M>
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Clone for LibinputSeat<C, D, G, S, T, M>
{
    fn clone(&self) -> LibinputSeat<C, D, G, S, T, M>
    {
        LibinputSeat {
            seat: unsafe { ffi::libinput_seat_ref(self.seat) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for LibinputSeat<C, D, G, S, T, M>
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> LibinputSeat<C, D, G, S, T, M>
{
    pub fn context(&self) -> LibinputContext<C, D, G, S, T, M> {
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
