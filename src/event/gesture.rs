use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

pub trait Gesture: AsRaw<ffi::libinput_event_gesture> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_gesture_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_gesture_get_time_usec(self.as_raw() as *mut _) }
    }

    fn finger_count(&self) -> i32 {
        unsafe { ffi::libinput_event_gesture_get_finger_count(self.as_raw() as *mut _) }
    }
}

impl<T: AsRaw<ffi::libinput_event_gesture>> Gesture for T {}

#[derive(Clone, Copy)]
pub enum GestureEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Swipe(GestureSwipeEvent<C, D, G, S, T>),
    Pinch(GesturePinchEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GestureEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GestureEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END =>
                GestureEvent::Swipe(GestureSwipeEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END =>
                GestureEvent::Pinch(GesturePinchEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GestureEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw(),
            GestureEvent::Pinch(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        match *self {
            GestureEvent::Swipe(ref mut event) => event.as_raw_mut(),
            GestureEvent::Pinch(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum GestureSwipeEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Begin(GestureSwipeBeginEvent<C, D, G, S, T>),
    Update(GestureSwipeUpdateEvent<C, D, G, S, T>),
    End(GestureSwipeEndEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GestureSwipeEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GestureSwipeEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN => GestureSwipeEvent::Begin(GestureSwipeBeginEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE => GestureSwipeEvent::Update(GestureSwipeUpdateEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END =>
                GestureSwipeEvent::End(GestureSwipeEndEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END =>
                panic!("Tried to make GestureSwipeEvent from Pinch event"),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GestureSwipeEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw(),
            GestureSwipeEvent::Update(ref event) => event.as_raw(),
            GestureSwipeEvent::End(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        match *self {
            GestureSwipeEvent::Begin(ref mut event) => event.as_raw_mut(),
            GestureSwipeEvent::Update(ref mut event) => event.as_raw_mut(),
            GestureSwipeEvent::End(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub enum GesturePinchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Begin(GesturePinchBeginEvent<C, D, G, S, T>),
    Update(GesturePinchUpdateEvent<C, D, G, S, T>),
    End(GesturePinchEndEvent<C, D, G, S, T>),
}

pub trait GesturePinch: AsRaw<ffi::libinput_event_gesture> {
    pub fn scale(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_scale(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinch for GesturePinchEvent<C, D, G, S, T> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GesturePinchEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GesturePinchEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        let base = ffi::libinput_event_gesture_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END =>
                panic!("Tried to make GesturePinchEvent from Swipe event"),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN =>
                GesturePinchEvent::Begin(GesturePinchBeginEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE =>
                GesturePinchEvent::Update(GesturePinchUpdateEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END =>
                GesturePinchEvent::End(GesturePinchEndEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GesturePinchEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw(),
            GesturePinchEvent::Update(ref event) => event.as_raw(),
            GesturePinchEvent::End(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        match *self {
            GesturePinchEvent::Begin(ref mut event) => event.as_raw_mut(),
            GesturePinchEvent::Update(ref mut event) => event.as_raw_mut(),
            GesturePinchEvent::End(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct GestureSwipeBeginEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GestureSwipeBeginEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GestureSwipeBeginEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GestureSwipeBeginEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GestureSwipeBeginEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct GestureSwipeUpdateEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GestureSwipeUpdateEvent<C, D, G, S, T> {
    pub fn dx(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dx(self.as_raw() as *mut _) }
    }

    pub fn dx_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dx_unaccelerated(self.as_raw() as *mut _) }
    }

    pub fn dy(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dy(self.as_raw() as *mut _) }
    }

    pub fn dy_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dy_unaccelerated(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GestureSwipeUpdateEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GestureSwipeUpdateEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GestureSwipeUpdateEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GestureSwipeUpdateEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct GestureSwipeEndEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GestureSwipeEndEvent<C, D, G, S, T> {
    pub fn cancelled(&self) -> bool {
        unsafe { ffi::libinput_event_gesture_get_cancelled(self.as_raw() as *mut _) == 1 }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GestureSwipeEndEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GestureSwipeEndEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GestureSwipeEndEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GestureSwipeEndEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct GesturePinchBeginEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinch for GesturePinchBeginEvent<C, D, G, S, T> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GesturePinchBeginEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GesturePinchBeginEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GesturePinchBeginEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GesturePinchBeginEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct GesturePinchUpdateEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinch for GesturePinchUpdateEvent<C, D, G, S, T> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinchUpdateEvent<C, D, G, S, T> {
    pub fn dx(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dx(self.as_raw() as *mut _) }
    }

    pub fn dx_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dx_unaccelerated(self.as_raw() as *mut _) }
    }

    pub fn dy(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dy(self.as_raw() as *mut _) }
    }

    pub fn dy_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_dy_unaccelerated(self.as_raw() as *mut _) }
    }

    pub fn angle_delta(&self) -> f64 {
        unsafe { ffi::libinput_event_gesture_get_angle_delta(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GesturePinchUpdateEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GesturePinchUpdateEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GesturePinchUpdateEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GesturePinchUpdateEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct GesturePinchEndEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinch for GesturePinchEndEvent<C, D, G, S, T> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> GesturePinchEndEvent<C, D, G, S, T> {
    pub fn cancelled(&self) -> bool {
        unsafe { ffi::libinput_event_gesture_get_cancelled(self.as_raw() as *mut _) == 1 }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for GesturePinchEndEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_gesture_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_gesture> for GesturePinchEndEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_gesture) -> Self {
        GesturePinchEndEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_gesture> for GesturePinchEndEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_gesture {
        self.event
    }
}
