use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

pub trait SwitchEventTrait: AsRaw<ffi::libinput_event_switch> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_switch_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_switch_get_time_usec(self.as_raw() as *mut _) }
    }
}

#[derive(Clone, Copy)]
pub enum SwitchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Toggle(SwitchToggleEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for SwitchEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_switch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_switch> for SwitchEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_switch) -> Self {
        let base = ffi::libinput_event_switch_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE =>
                SwitchEvent::Toggle(SwitchToggleEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_switch> for SwitchEvent<C, D, G, S, T, M> {
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
pub enum Switch {
    Lid
}

#[derive(Clone, Copy)]
pub enum SwitchState {
    Off,
    On,
}

#[derive(Clone, Copy)]
pub struct SwitchToggleEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_switch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> SwitchToggleEvent<C, D, G, S, T, M> {
    pub fn switch(&self) -> Switch {
        match unsafe { ffi::libinput_event_switch_get_switch(self.as_raw() as *mut _) } {
            ffi::libinput_switch::LIBINPUT_SWITCH_LID => Switch::Lid,
        }
    }

    pub fn switch_state(&self) -> SwitchState {
        match unsafe { ffi::libinput_event_switch_get_switch_state(self.as_raw() as *mut _) } {
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_OFF => SwitchState::Off,
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_ON => SwitchState::On,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for SwitchToggleEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_switch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_switch> for SwitchToggleEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_switch) -> Self {
        SwitchToggleEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_switch> for SwitchToggleEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_switch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_switch {
        self.event
    }
}
