use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

use std::marker::PhantomData;

pub trait SwitchEventTrait: AsRaw<ffi::libinput_event_switch> {
    ffi_func!(time, ffi::libinput_event_switch_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_switch_get_time_usec, u64);
}

impl<T: AsRaw<ffi::libinput_event_switch>> SwitchEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Toggle(SwitchToggleEvent<C, D, G, S, T, M>),
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
    fn as_raw(&self) -> *const ffi::libinput_event_switch {
        match *self {
            SwitchEvent::Toggle(ref event) => event.as_raw(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Switch {
    Lid
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    Off,
    On,
}

ffi_event_struct!(SwitchToggleEvent, ffi::libinput_event_switch, ffi::libinput_event_switch_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> SwitchToggleEvent<C, D, G, S, T, M> {
    pub fn switch(&self) -> Switch {
        match unsafe { ffi::libinput_event_switch_get_switch(self.as_raw_mut()) } {
            ffi::libinput_switch::LIBINPUT_SWITCH_LID => Switch::Lid,
        }
    }

    pub fn switch_state(&self) -> SwitchState {
        match unsafe { ffi::libinput_event_switch_get_switch_state(self.as_raw_mut()) } {
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_OFF => SwitchState::Off,
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_ON => SwitchState::On,
        }
    }
}
