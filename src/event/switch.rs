use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

pub trait SwitchEventTrait: AsRaw<ffi::libinput_event_switch> {
    ffi_func!(fn time, ffi::libinput_event_switch_get_time, u32);
    ffi_func!(fn time_usec, ffi::libinput_event_switch_get_time_usec, u64);

    fn into_switch_event(self) -> SwitchEvent where Self: Sized {
        unsafe { SwitchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_switch>> SwitchEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchEvent {
    Toggle(SwitchToggleEvent),
}

impl EventTrait for SwitchEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            SwitchEvent::Toggle(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_switch> for SwitchEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_switch) -> Self {
        let base = ffi::libinput_event_switch_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE =>
                SwitchEvent::Toggle(SwitchToggleEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_switch> for SwitchEvent {
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

ffi_event_struct!(struct SwitchToggleEvent, ffi::libinput_event_switch, ffi::libinput_event_switch_get_base_event);

impl SwitchToggleEvent {
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
