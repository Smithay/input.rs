use ::ffi;
use ::{FromRaw, AsRaw};
use super::{EventTrait, ButtonState};

use std::marker::PhantomData;

mod mode_group;
pub use self::mode_group::*;

pub trait TabletPadEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_tablet_pad> {
    ffi_func!(time, ffi::libinput_event_tablet_pad_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_tablet_pad_get_time_usec, u64);
    ffi_func!(mode, ffi::libinput_event_tablet_pad_get_mode, u32);

    fn mode_group(&self) -> TabletPadModeGroup<C, D, G, S, T, M> {
        unsafe { TabletPadModeGroup::from_raw(ffi::libinput_event_tablet_pad_get_mode_group(self.as_raw_mut())) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static, R: AsRaw<ffi::libinput_event_tablet_pad>> TabletPadEventTrait<C, D, G, S, T, M> for R {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TabletPadEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Button(TabletPadButtonEvent<C, D, G, S, T, M>),
    Ring(TabletPadRingEvent<C, D, G, S, T, M>),
    Strip(TabletPadStripEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        let base = ffi::libinput_event_tablet_pad_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_BUTTON =>
                TabletPadEvent::Button(TabletPadButtonEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_RING =>
                TabletPadEvent::Ring(TabletPadRingEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_STRIP =>
                TabletPadEvent::Strip(TabletPadStripEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        match *self {
            TabletPadEvent::Button(ref event) => event.as_raw(),
            TabletPadEvent::Ring(ref event) => event.as_raw(),
            TabletPadEvent::Strip(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(TabletPadButtonEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadButtonEvent<C, D, G, S, T, M> {
    ffi_func!(pub button_number, ffi::libinput_event_tablet_pad_get_button_number, u32);

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_tablet_pad_get_button_state(self.as_raw_mut()) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RingAxisSource {
    Unknown,
    Finger,
}

ffi_event_struct!(TabletPadRingEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadRingEvent<C, D, G, S, T, M> {
    ffi_func!(pub number, ffi::libinput_event_tablet_pad_get_ring_number, u32);
    ffi_func!(pub position, ffi::libinput_event_tablet_pad_get_ring_position, f64);

    pub fn source(&self) -> RingAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_ring_source(self.as_raw_mut()) } {
            ffi::libinput_tablet_pad_ring_axis_source::LIBINPUT_TABLET_PAD_RING_SOURCE_UNKNOWN => RingAxisSource::Unknown,
            ffi::libinput_tablet_pad_ring_axis_source::LIBINPUT_TABLET_PAD_RING_SOURCE_FINGER => RingAxisSource::Finger,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StripAxisSource {
    Unknown,
    Finger,
}

ffi_event_struct!(TabletPadStripEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadStripEvent<C, D, G, S, T, M> {
    ffi_func!(pub number, ffi::libinput_event_tablet_pad_get_strip_number, u32);
    ffi_func!(pub position, ffi::libinput_event_tablet_pad_get_strip_position, f64);

    pub fn source(&self) -> StripAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_strip_source(self.as_raw_mut()) } {
            ffi::libinput_tablet_pad_strip_axis_source::LIBINPUT_TABLET_PAD_STRIP_SOURCE_UNKNOWN => StripAxisSource::Unknown,
            ffi::libinput_tablet_pad_strip_axis_source::LIBINPUT_TABLET_PAD_STRIP_SOURCE_FINGER => StripAxisSource::Finger,
        }
    }
}
