use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub enum TabletPadEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Button(TabletPadButtonEvent<C, D, G, S, T>),
    Ring(TabletPadRingEvent<C, D, G, S, T>),
    Strip(TabletPadStripEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TabletPadEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent<C, D, G, S, T> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent<C, D, G, S, T> {
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
pub struct TabletPadButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TabletPadButtonEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadButtonEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadButtonEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadButtonEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TabletPadRingEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static>  {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TabletPadRingEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadRingEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadRingEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadRingEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TabletPadStripEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TabletPadStripEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_pad_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_tablet_pad> for TabletPadStripEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad) -> Self {
        TabletPadStripEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_tablet_pad> for TabletPadStripEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_pad {
        self.event
    }
}
