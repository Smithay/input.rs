use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

pub trait TouchEventTrait: AsRaw<ffi::libinput_event_touch> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_touch_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_touch_get_time_usec(self.as_raw() as *mut _) }
    }
}

impl<T: AsRaw<ffi::libinput_event_touch>> TouchEventTrait for T {}

pub trait TouchEventSlot: AsRaw<ffi::libinput_event_touch> {
    fn seat_slot(&self) -> u32 {
        unsafe { ffi::libinput_event_touch_get_seat_slot(self.as_raw() as *mut _) as u32 }
    }

    fn slot(&self) -> Option<u32> {
        match unsafe { ffi::libinput_event_touch_get_slot(self.as_raw() as *mut _) } {
            x if x >= 0 => Some(x as u32),
            -1 => None,
            _ => panic!("libinput_event_touch_get_slot returned undocumentated value!"),
        }
    }
}

pub trait TouchEventPosition: AsRaw<ffi::libinput_event_touch> {
    fn x(&self) -> f64 {
        unsafe { ffi::libinput_event_touch_get_x(self.as_raw() as *mut _) }
    }

    fn x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_x_transformed(self.as_raw() as *mut _, width) }
    }

    fn y(&self) -> f64 {
        unsafe { ffi::libinput_event_touch_get_y(self.as_raw() as *mut _) }
    }

    fn y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_y_transformed(self.as_raw() as *mut _, height) }
    }
}

#[derive(Clone, Copy)]
pub enum TouchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Down(TouchDownEvent<C, D, G, S, T, M>),
    Up(TouchUpEvent<C, D, G, S, T, M>),
    Motion(TouchMotionEvent<C, D, G, S, T, M>),
    Cancel(TouchCancelEvent<C, D, G, S, T, M>),
    Frame(TouchFrameEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchEvent<C, D, G, S, T, M> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchEvent<C, D, G, S, T, M> {
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
pub struct TouchDownEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchDownEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventPosition for TouchDownEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchDownEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchDownEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchDownEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchDownEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchUpEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchUpEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchUpEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchUpEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchUpEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchUpEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchMotionEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchMotionEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventPosition for TouchMotionEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchMotionEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchMotionEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchMotionEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchMotionEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchCancelEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchCancelEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchCancelEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchCancelEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchCancelEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchCancelEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TouchFrameEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TouchFrameEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_touch_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_touch> for TouchFrameEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_touch) -> Self {
        TouchFrameEvent {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_touch> for TouchFrameEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_touch {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_touch {
        self.event
    }
}
