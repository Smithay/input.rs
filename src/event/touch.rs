use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

pub trait TouchEventTrait: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(time, ffi::libinput_event_touch_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_touch_get_time_usec, u64);

    fn into_touch_event(self) -> TouchEvent where Self: Sized {
        unsafe { TouchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_touch>> TouchEventTrait for T {}

pub trait TouchEventSlot: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(seat_slot, ffi::libinput_event_touch_get_seat_slot, u32);

    fn slot(&self) -> Option<u32> {
        match unsafe { ffi::libinput_event_touch_get_slot(self.as_raw_mut()) } {
            x if x >= 0 => Some(x as u32),
            -1 => None,
            _ => panic!("libinput_event_touch_get_slot returned undocumentated value!"),
        }
    }
}

pub trait TouchEventPosition: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(x, ffi::libinput_event_touch_get_x, f64);
    ffi_func!(y, ffi::libinput_event_touch_get_y, f64);

    fn x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_x_transformed(self.as_raw_mut(), width) }
    }

    fn y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_y_transformed(self.as_raw_mut(), height) }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TouchEvent {
    Down(TouchDownEvent),
    Up(TouchUpEvent),
    Motion(TouchMotionEvent),
    Cancel(TouchCancelEvent),
    Frame(TouchFrameEvent),
}

impl EventTrait for TouchEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            TouchEvent::Down(ref event) => event.as_raw_event(),
            TouchEvent::Up(ref event) => event.as_raw_event(),
            TouchEvent::Motion(ref event) => event.as_raw_event(),
            TouchEvent::Cancel(ref event) => event.as_raw_event(),
            TouchEvent::Frame(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_touch> for TouchEvent {
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

impl AsRaw<ffi::libinput_event_touch> for TouchEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_touch {
        match *self {
            TouchEvent::Down(ref event) => event.as_raw(),
            TouchEvent::Up(ref event) => event.as_raw(),
            TouchEvent::Motion(ref event) => event.as_raw(),
            TouchEvent::Cancel(ref event) => event.as_raw(),
            TouchEvent::Frame(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(TouchDownEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchDownEvent {}

impl TouchEventPosition for TouchDownEvent {}

ffi_event_struct!(TouchUpEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchUpEvent {}

ffi_event_struct!(TouchMotionEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchMotionEvent {}

impl TouchEventPosition for TouchMotionEvent {}

ffi_event_struct!(TouchCancelEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchCancelEvent {}

ffi_event_struct!(TouchFrameEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);
