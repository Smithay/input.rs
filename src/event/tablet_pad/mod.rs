use ::ffi;
use ::{FromRaw, AsRaw};
use super::{Event, ButtonState};

use std::marker::PhantomData;

mod mode_group;
pub use self::mode_group::*;

pub trait TabletPadEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_tablet_pad> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_pad_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_tablet_pad_get_time_usec(self.as_raw() as *mut _) }
    }

    fn mode(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_pad_get_mode(self.as_raw() as *mut _) }
    }

    fn mode_group(&self) -> TabletPadModeGroup<C, D, G, S, T, M> {
        unsafe { TabletPadModeGroup::from_raw(ffi::libinput_event_tablet_pad_get_mode_group(self.as_raw() as *mut _)) }
    }
}

#[derive(Clone, Copy)]
pub enum TabletPadEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Button(TabletPadButtonEvent<C, D, G, S, T, M>),
    Ring(TabletPadRingEvent<C, D, G, S, T, M>),
    Strip(TabletPadStripEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletPadEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
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
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        match *self {
            TabletPadEvent::Button(ref event) => event.as_raw(),
            TabletPadEvent::Ring(ref event) => event.as_raw(),
            TabletPadEvent::Strip(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        match *self {
            TabletPadEvent::Button(ref mut event) => event.as_raw_mut(),
            TabletPadEvent::Ring(ref mut event) => event.as_raw_mut(),
            TabletPadEvent::Strip(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TabletPadButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadButtonEvent<C, D, G, S, T, M> {
    pub fn button_number(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_pad_get_button_number(self.as_raw() as *mut _) }
    }

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_tablet_pad_get_button_state(self.as_raw() as *mut _) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletPadButtonEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadButtonEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadButtonEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadButtonEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}

#[derive(Clone, Copy)]
pub enum RingAxisSource {
    Unknown,
    Finger,
}

#[derive(Clone, Copy)]
pub struct TabletPadRingEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadRingEvent<C, D, G, S, T, M> {
    pub fn number(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_pad_get_ring_number(self.as_raw() as *mut _) }
    }

    pub fn position(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_pad_get_ring_position(self.as_raw() as *mut _) }
    }

    pub fn source(&self) -> RingAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_ring_source(self.as_raw() as *mut _) } {
            ffi::libinput_tablet_pad_ring_axis_source::LIBINPUT_TABLET_PAD_RING_SOURCE_UNKNOWN => RingAxisSource::Unknown,
            ffi::libinput_tablet_pad_ring_axis_source::LIBINPUT_TABLET_PAD_RING_SOURCE_FINGER => RingAxisSource::Finger,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletPadRingEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadRingEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadRingEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadRingEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}

#[derive(Clone, Copy)]
pub enum StripAxisSource {
    Unknown,
    Finger,
}

#[derive(Clone, Copy)]
pub struct TabletPadStripEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletPadStripEvent<C, D, G, S, T, M> {
    pub fn number(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_pad_get_strip_number(self.as_raw() as *mut _) }
    }

    pub fn position(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_pad_get_strip_position(self.as_raw() as *mut _) }
    }

    pub fn source(&self) -> StripAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_strip_source(self.as_raw() as *mut _) } {
            ffi::libinput_tablet_pad_strip_axis_source::LIBINPUT_TABLET_PAD_STRIP_SOURCE_UNKNOWN => StripAxisSource::Unknown,
            ffi::libinput_tablet_pad_strip_axis_source::LIBINPUT_TABLET_PAD_STRIP_SOURCE_FINGER => StripAxisSource::Finger,
        }
    }}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletPadStripEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadStripEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadStripEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadStripEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}
