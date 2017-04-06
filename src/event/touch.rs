use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

use std::marker::PhantomData;

pub trait TouchEventTrait: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(time, ffi::libinput_event_touch_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_touch_get_time_usec, u64);
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
pub enum TouchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Down(TouchDownEvent<C, D, G, S, T, M>),
    Up(TouchUpEvent<C, D, G, S, T, M>),
    Motion(TouchMotionEvent<C, D, G, S, T, M>),
    Cancel(TouchCancelEvent<C, D, G, S, T, M>),
    Frame(TouchFrameEvent<C, D, G, S, T, M>),
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchDownEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventPosition for TouchDownEvent<C, D, G, S, T, M> {}

ffi_event_struct!(TouchUpEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchUpEvent<C, D, G, S, T, M> {}

ffi_event_struct!(TouchMotionEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchMotionEvent<C, D, G, S, T, M> {}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventPosition for TouchMotionEvent<C, D, G, S, T, M> {}

ffi_event_struct!(TouchCancelEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TouchEventSlot for TouchCancelEvent<C, D, G, S, T, M> {}

ffi_event_struct!(TouchFrameEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);
