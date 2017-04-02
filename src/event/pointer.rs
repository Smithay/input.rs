use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

pub trait Pointer: AsRaw<ffi::libinput_event_pointer> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_pointer_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_pointer_get_time_usec(self.as_raw() as *mut _) }
    }
}

impl<T: AsRaw<ffi::libinput_event_pointer>> Pointer for T {}

#[derive(Clone, Copy)]
pub enum PointerEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Motion(PointerMotionEvent<C, D, G, S, T>),
    MotionAbsolute(PointerMotionAbsoluteEvent<C, D, G, S, T>),
    Button(PointerButtonEvent<C, D, G, S, T>),
    Axis(PointerAxisEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for PointerEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_keyboard_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_pointer> for PointerEvent<C, D, G, S, T> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_pointer> for PointerEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        match *self {
            PointerEvent::Motion(ref event) => event.as_raw(),
            PointerEvent::MotionAbsolute(ref event) => event.as_raw(),
            PointerEvent::Button(ref event) => event.as_raw(),
            PointerEvent::Axis(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_pointer {
        match *self {
            PointerEvent::Motion(ref mut event) => event.as_raw_mut(),
            PointerEvent::MotionAbsolute(ref mut event) => event.as_raw_mut(),
            PointerEvent::Button(ref mut event) => event.as_raw_mut(),
            PointerEvent::Axis(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct PointerMotionEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_pointer,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> PointerMotionEvent<C, D, G, S, T> {
    pub fn dx(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_dx(self.as_raw() as *mut _) }
    }

    pub fn dx_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_dx_unaccelerated(self.as_raw() as *mut _) }
    }

    pub fn dy(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_dy(self.as_raw() as *mut _) }
    }

    pub fn dy_unaccelerated(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_dy_unaccelerated(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for PointerMotionEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_pointer_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_pointer> for PointerMotionEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer) -> Self {
        PointerMotionEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_pointer> for PointerMotionEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_pointer {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct PointerMotionAbsoluteEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_pointer,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> PointerMotionAbsoluteEvent<C, D, G, S, T> {
    pub fn absolute_x(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_x(self.as_raw() as *mut _) }
    }

    pub fn absolute_x_transformed(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_x_transformed(self.as_raw() as *mut _) }
    }

    pub fn absolute_y(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_y(self.as_raw() as *mut _) }
    }

    pub fn absolute_y_transformed(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_y_transformed(self.as_raw() as *mut _) }
    }
}
impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for PointerMotionAbsoluteEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_pointer_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_pointer> for PointerMotionAbsoluteEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer) -> Self {
        PointerMotionAbsoluteEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_pointer> for PointerMotionAbsoluteEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_pointer {
        self.event
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ButtonState {
    Pressed,
    Released,
}

#[derive(Clone, Copy)]
pub struct PointerButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_pointer,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> PointerButtonEvent<C, D, G, S, T> {
    pub fn button(&self) -> u32 {
        unsafe { ffi::libinput_event_pointer_get_button(self.as_raw() as *mut _) }
    }

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_pointer_get_button_state(self.as_raw() as *mut _) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }

    pub fn seat_button_count() -> u32 {
        unsafe { ffi::libinput_event_keyboard_get_seat_button_count(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for PointerButtonEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_pointer_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_pointer> for PointerButtonEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer) -> Self {
        PointerButtonEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_pointer> for PointerButtonEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_pointer {
        self.event
    }
}

#[derive(Debug, Clone, Copy)]
pub enum AxisSource {
    Wheel,
    Finger,
    Continuous,
    WheelTilt,
}

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    Vertical,
    Horizontal,
}

#[derive(Clone, Copy)]
pub struct PointerAxisEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_pointer,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> PointerAxisEvent<C, D, G, S, T> {
    pub fn has_axis(&self, axis: Axis) -> bool {
        unsafe { ffi::libinput_event_pointer_has_axis(self.as_raw() as *mut _, match axis {
            Axis::Vertical => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
            Axis::Horizontal => ffi::libinput_pointer_axis::LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
        }) } != 0
    }

    pub fn axis_source(&self) -> AxisSource {
        match unsafe { ffi::libinput_event_pointer_get_axis_source(self.as_raw() as *mut _) } {
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_WHEEL => AxisSource::Wheel,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_FINGER => AxisSource::Finger,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_CONTINUOUS => AxisSource::Continuous,
            ffi::libinput_pointer_axis_source::LIBINPUT_POINTER_AXIS_SOURCE_WHEEL_TILT => AxisSource::WheelTilt,
        }
    }

    pub fn axis_value(&self) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_axis_value(self.as_raw() as *mut _) }
    }

    pub fn axis_value_discrete(&self) -> Option<f64> {
        match self.axis_source() {
            AxisSource::Continuous | AxisSource::Finger => None,
            _ => Some(unsafe { ffi::libinput_event_pointer_get_axis_value_discrete(self.as_raw() as *mut _) }), 
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for PointerAxisEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_pointer_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_pointer> for PointerAxisEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer) -> Self {
        PointerAxisEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_pointer> for PointerAxisEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_pointer {
        self.event
    }
}
