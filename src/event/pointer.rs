use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

use std::marker::PhantomData;

pub trait PointerEventTrait: AsRaw<ffi::libinput_event_pointer> {
    ffi_func!(time, ffi::libinput_event_pointer_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_pointer_get_time_usec, u64);
}

impl<T: AsRaw<ffi::libinput_event_pointer>> PointerEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PointerEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Motion(PointerMotionEvent<C, D, G, S, T, M>),
    MotionAbsolute(PointerMotionAbsoluteEvent<C, D, G, S, T, M>),
    Button(PointerButtonEvent<C, D, G, S, T, M>),
    Axis(PointerAxisEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_pointer> for PointerEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer) -> Self {
        let base = ffi::libinput_event_pointer_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION =>
                PointerEvent::Motion(PointerMotionEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE =>
                PointerEvent::MotionAbsolute(PointerMotionAbsoluteEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_BUTTON =>
                PointerEvent::Button(PointerButtonEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_AXIS =>
                PointerEvent::Axis(PointerAxisEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_pointer> for PointerEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        match *self {
            PointerEvent::Motion(ref event) => event.as_raw(),
            PointerEvent::MotionAbsolute(ref event) => event.as_raw(),
            PointerEvent::Button(ref event) => event.as_raw(),
            PointerEvent::Axis(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(PointerMotionEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PointerMotionEvent<C, D, G, S, T, M> {
    ffi_func!(pub dx, ffi::libinput_event_pointer_get_dx, f64);
    ffi_func!(pub dx_unaccelerated, ffi::libinput_event_pointer_get_dx_unaccelerated, f64);
    ffi_func!(pub dy, ffi::libinput_event_pointer_get_dy, f64);
    ffi_func!(pub dy_unaccelerated, ffi::libinput_event_pointer_get_dy_unaccelerated, f64);
}

ffi_event_struct!(PointerMotionAbsoluteEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PointerMotionAbsoluteEvent<C, D, G, S, T, M> {
    ffi_func!(pub absolute_x, ffi::libinput_event_pointer_get_absolute_x, f64);
    ffi_func!(pub absolute_y, ffi::libinput_event_pointer_get_absolute_y, f64);

    pub fn absolute_x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_x_transformed(self.as_raw_mut(), width) }
    }

    pub fn absolute_y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_y_transformed(self.as_raw_mut(), height) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    Pressed,
    Released,
}

ffi_event_struct!(PointerButtonEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PointerButtonEvent<C, D, G, S, T, M> {
    ffi_func!(pub button, ffi::libinput_event_pointer_get_button, u32);
    ffi_func!(pub seat_button_count, ffi::libinput_event_pointer_get_seat_button_count, u32);

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_pointer_get_button_state(self.as_raw_mut()) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisSource {
    Wheel,
    Finger,
    Continuous,
    WheelTilt,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Vertical,
    Horizontal,
}

ffi_event_struct!(PointerAxisEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> PointerAxisEvent<C, D, G, S, T, M> {
    pub fn has_axis(&self, axis: Axis) -> bool {
        unsafe { ffi::libinput_event_pointer_has_axis(self.as_raw_mut(), match axis {
            Axis::Vertical => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
            Axis::Horizontal => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
        }) != 0 }
    }

    pub fn axis_source(&self) -> AxisSource {
        match unsafe { ffi::libinput_event_pointer_get_axis_source(self.as_raw_mut()) } {
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_WHEEL => AxisSource::Wheel,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_FINGER => AxisSource::Finger,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_CONTINUOUS => AxisSource::Continuous,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_WHEEL_TILT => AxisSource::WheelTilt,
        }
    }

    pub fn axis_value(&self, axis: Axis) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_axis_value(self.as_raw_mut(), match axis {
            Axis::Vertical => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
            Axis::Horizontal => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
        }) }
    }

    pub fn axis_value_discrete(&self, axis: Axis) -> Option<f64> {
        match self.axis_source() {
            AxisSource::Continuous | AxisSource::Finger => None,
            _ =>
            Some(unsafe { ffi::libinput_event_pointer_get_axis_value_discrete(self.as_raw_mut(),
                match axis {
                    Axis::Vertical => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
                    Axis::Horizontal => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
                })
            }),
        }
    }
}
