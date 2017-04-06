use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

pub trait GestureEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(time, ffi::libinput_event_gesture_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_gesture_get_time_usec, u64);
    ffi_func!(finger_count, ffi::libinput_event_gesture_get_finger_count, i32);

    fn into_gesture_event(self) -> GestureEvent<C, D, G, S, T, M> where Self: Sized {
        unsafe { GestureEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static, R: AsRaw<ffi::libinput_event_gesture>> GestureEventTrait<C, D, G, S, T, M> for R {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GestureEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Swipe(GestureSwipeEvent<C, D, G, S, T, M>),
    Pinch(GesturePinchEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> EventTrait<C, D, G, S, T, M> for GestureEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw_event(),
            GestureEvent::Pinch(ref event) => event.as_raw_event(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_gesture> for GestureEvent<C, D, G, S, T, M> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_gesture> for GestureEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw(),
            GestureEvent::Pinch(ref event) => event.as_raw(),
        }
    }
}

pub trait GestureEventCoordinates: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(dx, ffi::libinput_event_gesture_get_dx, f64);
    ffi_func!(dx_unaccelerated, ffi::libinput_event_gesture_get_dx_unaccelerated, f64);
    ffi_func!(dy, ffi::libinput_event_gesture_get_dy, f64);
    ffi_func!(dy_unaccelerated, ffi::libinput_event_gesture_get_dy_unaccelerated, f64);
}

pub trait GestureEndEvent: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(cancelled, ffi::libinput_event_gesture_get_cancelled, bool);
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GestureSwipeEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Begin(GestureSwipeBeginEvent<C, D, G, S, T, M>),
    Update(GestureSwipeUpdateEvent<C, D, G, S, T, M>),
    End(GestureSwipeEndEvent<C, D, G, S, T, M>),
}

pub trait GestureSwipeEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_gesture> {
    fn into_gesture_swipe_event(self) -> GestureSwipeEvent<C, D, G, S, T, M> where Self: Sized {
        unsafe { GestureSwipeEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureSwipeEventTrait<C, D, G, S, T, M> for GestureSwipeEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> EventTrait<C, D, G, S, T, M> for GestureSwipeEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw_event(),
            GestureSwipeEvent::Update(ref event) => event.as_raw_event(),
            GestureSwipeEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_gesture> for GestureSwipeEvent<C, D, G, S, T, M> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_gesture> for GestureSwipeEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw(),
            GestureSwipeEvent::Update(ref event) => event.as_raw(),
            GestureSwipeEvent::End(ref event) => event.as_raw(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GesturePinchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Begin(GesturePinchBeginEvent<C, D, G, S, T, M>),
    Update(GesturePinchUpdateEvent<C, D, G, S, T, M>),
    End(GesturePinchEndEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> EventTrait<C, D, G, S, T, M> for GesturePinchEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw_event(),
            GesturePinchEvent::Update(ref event) => event.as_raw_event(),
            GesturePinchEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

pub trait GesturePinchEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(scale, ffi::libinput_event_gesture_get_scale, f64);

    fn into_gesture_pinch_event(self) -> GesturePinchEvent<C, D, G, S, T, M> where Self: Sized {
        unsafe { GesturePinchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GesturePinchEventTrait<C, D, G, S, T, M> for GesturePinchEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_gesture> for GesturePinchEvent<C, D, G, S, T, M> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_gesture> for GesturePinchEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw(),
            GesturePinchEvent::Update(ref event) => event.as_raw(),
            GesturePinchEvent::End(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(GestureSwipeBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureSwipeEventTrait<C, D, G, S, T, M> for GestureSwipeBeginEvent<C, D, G, S, T, M> {}

ffi_event_struct!(GestureSwipeUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureSwipeEventTrait<C, D, G, S, T, M> for GestureSwipeUpdateEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureEventCoordinates for GestureSwipeUpdateEvent<C, D, G, S, T, M> {}

ffi_event_struct!(GestureSwipeEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureEndEvent for GestureSwipeEndEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureSwipeEventTrait<C, D, G, S, T, M> for GestureSwipeEndEvent<C, D, G, S, T, M> {}

ffi_event_struct!(GesturePinchBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GesturePinchEventTrait<C, D, G, S, T, M> for GesturePinchBeginEvent<C, D, G, S, T, M> {}

ffi_event_struct!(GesturePinchUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GesturePinchEventTrait<C, D, G, S, T, M> for GesturePinchUpdateEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureEventCoordinates for GesturePinchUpdateEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GesturePinchUpdateEvent<C, D, G, S, T, M> {
    ffi_func!(pub angle_delta, ffi::libinput_event_gesture_get_angle_delta, f64);
}

ffi_event_struct!(GesturePinchEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GesturePinchEventTrait<C, D, G, S, T, M> for GesturePinchEndEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> GestureEndEvent for GesturePinchEndEvent<C, D, G, S, T, M> {}
