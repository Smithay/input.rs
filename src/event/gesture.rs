use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

pub trait GestureEventTrait: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(time, ffi::libinput_event_gesture_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_gesture_get_time_usec, u64);
    ffi_func!(finger_count, ffi::libinput_event_gesture_get_finger_count, i32);

    fn into_gesture_event(self) -> GestureEvent where Self: Sized {
        unsafe { GestureEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_gesture>> GestureEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GestureEvent {
    Swipe(GestureSwipeEvent),
    Pinch(GesturePinchEvent),
}

impl EventTrait for GestureEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureEvent::Swipe(ref event) => event.as_raw_event(),
            GestureEvent::Pinch(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_gesture> for GestureEvent {
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

impl AsRaw<ffi::libinput_event_gesture> for GestureEvent {
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
pub enum GestureSwipeEvent {
    Begin(GestureSwipeBeginEvent),
    Update(GestureSwipeUpdateEvent),
    End(GestureSwipeEndEvent),
}

pub trait GestureSwipeEventTrait: AsRaw<ffi::libinput_event_gesture> {
    fn into_gesture_swipe_event(self) -> GestureSwipeEvent where Self: Sized {
        unsafe { GestureSwipeEvent::from_raw(self.as_raw_mut()) }
    }
}

impl GestureSwipeEventTrait for GestureSwipeEvent {}

impl EventTrait for GestureSwipeEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw_event(),
            GestureSwipeEvent::Update(ref event) => event.as_raw_event(),
            GestureSwipeEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_gesture> for GestureSwipeEvent {
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

impl AsRaw<ffi::libinput_event_gesture> for GestureSwipeEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GestureSwipeEvent::Begin(ref event) => event.as_raw(),
            GestureSwipeEvent::Update(ref event) => event.as_raw(),
            GestureSwipeEvent::End(ref event) => event.as_raw(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GesturePinchEvent {
    Begin(GesturePinchBeginEvent),
    Update(GesturePinchUpdateEvent),
    End(GesturePinchEndEvent),
}

impl EventTrait for GesturePinchEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw_event(),
            GesturePinchEvent::Update(ref event) => event.as_raw_event(),
            GesturePinchEvent::End(ref event) => event.as_raw_event(),
        }
    }
}

pub trait GesturePinchEventTrait: AsRaw<ffi::libinput_event_gesture> {
    ffi_func!(scale, ffi::libinput_event_gesture_get_scale, f64);

    fn into_gesture_pinch_event(self) -> GesturePinchEvent where Self: Sized {
        unsafe { GesturePinchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl GesturePinchEventTrait for GesturePinchEvent {}

impl FromRaw<ffi::libinput_event_gesture> for GesturePinchEvent {
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

impl AsRaw<ffi::libinput_event_gesture> for GesturePinchEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_gesture {
        match *self {
            GesturePinchEvent::Begin(ref event) => event.as_raw(),
            GesturePinchEvent::Update(ref event) => event.as_raw(),
            GesturePinchEvent::End(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(GestureSwipeBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureSwipeEventTrait for GestureSwipeBeginEvent {}

ffi_event_struct!(GestureSwipeUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureSwipeEventTrait for GestureSwipeUpdateEvent {}

impl GestureEventCoordinates for GestureSwipeUpdateEvent {}

ffi_event_struct!(GestureSwipeEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GestureEndEvent for GestureSwipeEndEvent {}

impl GestureSwipeEventTrait for GestureSwipeEndEvent {}

ffi_event_struct!(GesturePinchBeginEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchBeginEvent {}

ffi_event_struct!(GesturePinchUpdateEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchUpdateEvent {}

impl GestureEventCoordinates for GesturePinchUpdateEvent {}

impl GesturePinchUpdateEvent {
    ffi_func!(pub angle_delta, ffi::libinput_event_gesture_get_angle_delta, f64);
}

ffi_event_struct!(GesturePinchEndEvent, ffi::libinput_event_gesture, ffi::libinput_event_gesture_get_base_event);

impl GesturePinchEventTrait for GesturePinchEndEvent {}

impl GestureEndEvent for GesturePinchEndEvent {}
