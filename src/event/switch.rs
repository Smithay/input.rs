//! Switch event types

use super::EventTrait;
use {AsRaw, FromRaw};
use ffi;

/// Common functions all Switch-Events implement.
pub trait SwitchEventTrait: AsRaw<ffi::libinput_event_switch> {
    ffi_func!(
    /// The event time for this event
    fn time, ffi::libinput_event_switch_get_time, u32);
    ffi_func!(
    /// The event time for this event in microseconds
    fn time_usec, ffi::libinput_event_switch_get_time_usec, u64);

    /// Convert into a general `SwitchEvent` again
    fn into_switch_event(self) -> SwitchEvent
        where Self: Sized
    {
        unsafe { SwitchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_switch>> SwitchEventTrait for T {}

/// A switch related `Event`
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SwitchEvent {
    /// An event related a switch, that was toggled
    Toggle(SwitchToggleEvent),
}

impl EventTrait for SwitchEvent {
    #[doc(hidden)]
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
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE => {
                SwitchEvent::Toggle(SwitchToggleEvent::from_raw(event))
            }
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

/// Types of Switches
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Switch {
    /// Lid closing switch
    Lid,
}

/// State of a Switch
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SwitchState {
    /// Switch is off
    Off,
    /// Switch is on
    On,
}

ffi_event_struct!(
/// An event related a switch, that was toggled
struct SwitchToggleEvent, ffi::libinput_event_switch, ffi::libinput_event_switch_get_base_event);

impl SwitchToggleEvent {
    /// Return the switch that triggered this event.
    pub fn switch(&self) -> Switch {
        match unsafe { ffi::libinput_event_switch_get_switch(self.as_raw_mut()) } {
            ffi::libinput_switch::LIBINPUT_SWITCH_LID => Switch::Lid,
        }
    }

    /// Return the switch state that triggered this event.
    pub fn switch_state(&self) -> SwitchState {
        match unsafe { ffi::libinput_event_switch_get_switch_state(self.as_raw_mut()) } {
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_OFF => SwitchState::Off,
            ffi::libinput_switch_state::LIBINPUT_SWITCH_STATE_ON => SwitchState::On,
        }
    }
}
