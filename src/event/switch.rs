use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub enum SwitchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Toggle(SwitchToggleEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for SwitchEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_switch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_switch> for SwitchEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_switch) -> Self {
        let base = ffi::libinput_event_switch_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE =>
                SwitchEvent::Toggle(SwitchToggleEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_switch> for SwitchEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_switch {
        match *self {
            SwitchEvent::Toggle(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_switch {
        match *self {
            SwitchEvent::Toggle(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct SwitchToggleEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_switch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for SwitchToggleEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_switch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_switch> for SwitchToggleEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_switch) -> Self {
        SwitchToggleEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_switch> for SwitchToggleEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_switch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_switch {
        self.event
    }
}
