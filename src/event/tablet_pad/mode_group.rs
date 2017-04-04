use ::{ffi, FromRaw, AsRaw, Userdata};

use libc;

use std::marker::PhantomData;
use std::{mem, ptr};

pub struct TabletPadModeGroup<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>
{
    group: *mut ffi::libinput_tablet_pad_mode_group,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadModeGroup<C, D, G, S, T, M> {
    pub fn button_is_toggle(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_button_is_toggle(self.as_raw() as *mut _, button) != 0 }
    }

    pub fn index(&self) -> u32 {
        unsafe { ffi::libinput_tablet_pad_mode_group_get_index(self.as_raw() as *mut _) }
    }

    pub fn mode(&self) -> u32 {
        unsafe { ffi::libinput_tablet_pad_mode_group_get_mode(self.as_raw() as *mut _) }
    }

    pub fn number_of_modes(&self) -> u32 {
        unsafe { ffi::libinput_tablet_pad_mode_group_get_num_modes(self.as_raw() as *mut _) }
    }

    pub fn has_button(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_button(self.as_raw() as *mut _, button) != 0 }
    }

    pub fn has_ring(&self, ring: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_ring(self.as_raw() as *mut _, ring) != 0 }
    }

    pub fn has_strip(&self, strip: u32) -> bool {
        unsafe { ffi::libinput_tablet_pad_mode_group_has_strip(self.as_raw() as *mut _, strip) != 0 }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  FromRaw<ffi::libinput_tablet_pad_mode_group> for TabletPadModeGroup<C, D, G, S, T, M>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_tablet_pad_mode_group) -> TabletPadModeGroup<C, D, G, S, T, M>
    {
        TabletPadModeGroup {
            group: ffi::libinput_tablet_pad_mode_group_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  AsRaw<ffi::libinput_tablet_pad_mode_group> for TabletPadModeGroup<C, D, G, S, T, M>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_tablet_pad_mode_group {
        self.group as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_tablet_pad_mode_group {
        self.group as *mut _
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Userdata<M> for TabletPadModeGroup<C, D, G, S, T, M>
{
    fn userdata(&self) -> Option<&M> {
        unsafe {
            (ffi::libinput_tablet_pad_mode_group_get_user_data(self.group) as *const M).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut M> {
        unsafe {
            (ffi::libinput_tablet_pad_mode_group_get_user_data(self.group) as *mut M).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<M>) -> Option<M> {
        let old = unsafe {
            let ptr = ffi::libinput_tablet_pad_mode_group_get_user_data(self.group);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut M))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_tablet_pad_mode_group_set_user_data(self.group, match (*boxed).as_mut() {
                Some(value) => value as *mut M as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Clone for TabletPadModeGroup<C, D, G, S, T, M>
{
    fn clone(&self) -> TabletPadModeGroup<C, D, G, S, T, M>
    {
        TabletPadModeGroup {
            group: unsafe { ffi::libinput_tablet_pad_mode_group_ref(self.group) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for TabletPadModeGroup<C, D, G, S, T, M>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_tablet_pad_mode_group_get_user_data(self.group);
            if ffi::libinput_tablet_pad_mode_group_unref(self.group).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}
