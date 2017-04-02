use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub enum TouchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Down(TouchDownEvent<C, D, G, S, T>),
    Up(TouchUpEvent<C, D, G, S, T>),
    Motion(TouchMotionEvent<C, D, G, S, T>),
    Cancel(TouchCancelEvent<C, D, G, S, T>),
    Frame(TouchFrameEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        let base = ffi::libinput_event_touch_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN =>
                TouchEvent::Down(TouchDownEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP =>
                TouchEvent::Up(TouchUpEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION =>
                TouchEvent::Motion(TouchMotionEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL =>
                TouchEvent::Cancel(TouchCancelEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME =>
                TouchEvent::Frame(TouchFrameEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        match *self {
            TouchEvent::Down(ref event) => event.as_raw(),
            TouchEvent::Up(ref event) => event.as_raw(),
            TouchEvent::Motion(ref event) => event.as_raw(),
            TouchEvent::Cancel(ref event) => event.as_raw(),
            TouchEvent::Frame(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        match *self {
            TouchEvent::Down(ref mut event) => event.as_raw_mut(),
            TouchEvent::Up(ref mut event) => event.as_raw_mut(),
            TouchEvent::Motion(ref mut event) => event.as_raw_mut(),
            TouchEvent::Cancel(ref mut event) => event.as_raw_mut(),
            TouchEvent::Frame(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TouchDownEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchDownEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchDownEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchDownEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchDownEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchUpEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchUpEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchUpEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchUpEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchUpEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchMotionEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchMotionEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchMotionEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchMotionEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchMotionEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchCancelEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchCancelEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchCancelEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchCancelEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchCancelEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchFrameEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for TouchFrameEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_touch> for TouchFrameEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchFrameEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_touch> for TouchFrameEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}
