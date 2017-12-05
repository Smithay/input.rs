//! Pointer event types

use super::EventTrait;
use {AsRaw, Context, FromRaw};
use ffi;

/// Common functions for all Pointer-Events implement.
pub trait PointerEventTrait: AsRaw<ffi::libinput_event_pointer> + Context {
    ffi_func!(
    /// The event time for this event
    fn time, ffi::libinput_event_pointer_get_time, u32);
    ffi_func!(
    /// The event time for this event in microseconds
    fn time_usec, ffi::libinput_event_pointer_get_time_usec, u64);

    /// Convert into a general `TouchEvent` again
    fn into_pointer_event(self) -> PointerEvent
    where
        Self: Sized,
    {
        unsafe { PointerEvent::from_raw(self.as_raw_mut(), self.context()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_pointer> + Context> PointerEventTrait for T {}

/// A pointer related `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum PointerEvent {
    /// An event related to moving a pointer
    Motion(PointerMotionEvent),
    /// An event related to absolute pointer movement
    MotionAbsolute(PointerMotionAbsoluteEvent),
    /// An event related to button pressed on a pointer device
    Button(PointerButtonEvent),
    /// An event related to moving axis on a pointer device
    Axis(PointerAxisEvent),
}

impl EventTrait for PointerEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            PointerEvent::Motion(ref event) => event.as_raw_event(),
            PointerEvent::MotionAbsolute(ref event) => event.as_raw_event(),
            PointerEvent::Button(ref event) => event.as_raw_event(),
            PointerEvent::Axis(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_pointer> for PointerEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_pointer, context: &::context::Libinput) -> Self {
        let base = ffi::libinput_event_pointer_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION => {
                PointerEvent::Motion(PointerMotionEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE => {
                PointerEvent::MotionAbsolute(PointerMotionAbsoluteEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_BUTTON => {
                PointerEvent::Button(PointerButtonEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_AXIS => {
                PointerEvent::Axis(PointerAxisEvent::from_raw(event, context))
            }
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_pointer> for PointerEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_pointer {
        match *self {
            PointerEvent::Motion(ref event) => event.as_raw(),
            PointerEvent::MotionAbsolute(ref event) => event.as_raw(),
            PointerEvent::Button(ref event) => event.as_raw(),
            PointerEvent::Axis(ref event) => event.as_raw(),
        }
    }
}

impl Context for PointerEvent {
    fn context(&self) -> &::Libinput {
        match *self {
            PointerEvent::Motion(ref event) => event.context(),
            PointerEvent::MotionAbsolute(ref event) => event.context(),
            PointerEvent::Button(ref event) => event.context(),
            PointerEvent::Axis(ref event) => event.context(),
        }
    }
}

ffi_event_struct!(
/// An event related to moving a pointer
struct PointerMotionEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl PointerMotionEvent {
    ffi_func!(
    /// Return the delta between the last event and the current event.
    ///
    /// If a device employs pointer acceleration, the delta returned by this
    /// function is the accelerated delta.
    ///
    /// Relative motion deltas are to be interpreted as pixel movement of a
    /// standardized mouse. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    pub fn dx, ffi::libinput_event_pointer_get_dx, f64);
    ffi_func!(
    /// Return the relative delta of the unaccelerated motion vector of the
    /// current event.
    ///
    /// Relative unaccelerated motion deltas are raw device coordinates. Note that
    /// these coordinates are subject to the device's native resolution. Touchpad
    /// coordinates represent raw device coordinates in the X resolution of the
    /// touchpad. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    ///
    /// Any rotation applied to the device also applies to unaccelerated motion
    /// (see `Device::rotation_set_angle`).
    pub fn dx_unaccelerated, ffi::libinput_event_pointer_get_dx_unaccelerated, f64);
    ffi_func!(
    /// Return the delta between the last event and the current event.
    ///
    /// If a device employs pointer acceleration, the delta returned by this
    /// function is the accelerated delta.
    ///
    /// Relative motion deltas are to be interpreted as pixel movement of a
    /// standardized mouse. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    pub fn dy, ffi::libinput_event_pointer_get_dy, f64);
    ffi_func!(
    /// Return the relative delta of the unaccelerated motion vector of the
    /// current event.
    ///
    /// Relative unaccelerated motion deltas are raw device coordinates. Note that
    /// these coordinates are subject to the device's native resolution. Touchpad
    /// coordinates represent raw device coordinates in the X resolution of the
    /// touchpad. See [Normalization of relative motion](https://wayland.freedesktop.org/libinput/doc/latest/motion_normalization.html)
    /// for more details.
    ///
    /// Any rotation applied to the device also applies to unaccelerated motion
    /// (see `Device::rotation_set_angle`).
    pub fn dy_unaccelerated, ffi::libinput_event_pointer_get_dy_unaccelerated, f64);
}

ffi_event_struct!(
/// An event related to absolute pointer movement
struct PointerMotionAbsoluteEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl PointerMotionAbsoluteEvent {
    ffi_func!(
    /// Return the current absolute x coordinate of the pointer event, in mm from
    /// the top left corner of the device.
    ///
    /// To get the corresponding output screen coordinate, use
    /// `absolute_x_transformed`.
    pub fn absolute_x, ffi::libinput_event_pointer_get_absolute_x, f64);
    ffi_func!(
    /// Return the current absolute y coordinate of the pointer event, in mm from
    /// the top left corner of the device.
    ///
    /// To get the corresponding output screen coordinate, use
    /// `absolute_y_transformed`.
    pub fn absolute_y, ffi::libinput_event_pointer_get_absolute_y, f64);

    /// Return the current absolute x coordinate of the pointer event, transformed
    /// to screen coordinates.
    ///
    /// ## Arguments
    ///
    /// - width - The current output screen width
    pub fn absolute_x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_x_transformed(self.as_raw_mut(), width) }
    }

    /// Return the current absolute y coordinate of the pointer event, transformed
    /// to screen coordinates.
    ///
    /// ## Arguments
    ///
    /// - height - The current output screen height
    pub fn absolute_y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_pointer_get_absolute_y_transformed(self.as_raw_mut(), height) }
    }
}

/// State of a Button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ButtonState {
    /// Button is pressed
    Pressed,
    /// Button is released
    Released,
}

ffi_event_struct!(
/// An event related to button pressed on a pointer device
struct PointerButtonEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl PointerButtonEvent {
    ffi_func!(
    /// Return the button that triggered this event.
    pub fn button, ffi::libinput_event_pointer_get_button, u32);
    ffi_func!(
    /// For the button returns the total number of buttons pressed on all devices
    /// on the associated seat after the event was triggered.
    pub fn seat_button_count, ffi::libinput_event_pointer_get_seat_button_count, u32);

    /// Return the button state that triggered this event.
    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_pointer_get_button_state(self.as_raw_mut()) } {
            ffi::libinput_button_state_LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state_LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
            _ => panic!("libinput returned invalid 'libinput_button_state'"),
        }
    }
}

/// The source for a `PointerAxisEvent`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AxisSource {
    /// The event is caused by the rotation of a wheel.
    Wheel,
    /// The event is caused by the movement of one or more fingers on a device.
    Finger,
    /// The event is caused by the motion of some device.
    Continuous,
    /// The event is caused by the tilting of a mouse wheel rather than its rotation.
    ///
    /// This method is commonly used on mice without separate horizontal scroll wheels.
    WheelTilt,
}

/// Axes on a device with the pointer capability that are not  x or y coordinates.
///
/// The two scroll axes `Vertical` and `Horizontal` are engaged separately,
/// depending on the device. libinput provides some scroll direction locking but
/// it is up to the caller to determine which axis is needed and appropriate in
/// the current interaction
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    /// Vertical axis
    Vertical,
    /// Horizontal axis
    Horizontal,
}

ffi_event_struct!(
/// An event related to moving axis on a pointer device
struct PointerAxisEvent, ffi::libinput_event_pointer, ffi::libinput_event_pointer_get_base_event);

impl PointerAxisEvent {
    /// Check if the event has a valid value for the given axis.
    ///
    /// If this function returns non-zero for an axis and `axis_value` returns a
    /// value of 0, the event is a scroll stop event.
    pub fn has_axis(&self, axis: Axis) -> bool {
        unsafe {
            ffi::libinput_event_pointer_has_axis(
                self.as_raw_mut(),
                match axis {
                    Axis::Vertical => ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
                    Axis::Horizontal => ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
                },
            ) != 0
        }
    }

    /// Return the source for a given axis event.
    ///
    /// Axis events (scroll events) can be caused by a hardware item such as a
    /// scroll wheel or emulated from other input sources, such as two-finger or
    /// edge scrolling on a touchpad.
    ///
    /// If the source is `Finger`, libinput guarantees that a scroll sequence is
    /// terminated with a scroll value of 0. A caller may use this information to
    /// decide on whether kinetic scrolling should be triggered on this scroll
    /// sequence. The coordinate system is identical to the cursor movement, i.e.
    /// a scroll value of 1 represents the equivalent relative motion of 1.
    ///
    /// If the source is `Wheel`, no terminating event is guaranteed (though it
    /// may happen). Scrolling is in discrete steps, the value is the angle the
    /// wheel moved in degrees. The default is 15 degrees per wheel click, but
    /// some mice may have differently grained wheels. It is up to the caller how
    /// to interpret such different step sizes.
    ///
    /// If the source is `Continuous`, no terminating event is guaranteed (though
    /// it may happen). The coordinate system is identical to the cursor movement,
    /// i.e. a scroll value of 1 represents the equivalent relative motion of 1.
    ///
    /// If the source is `WheelTilt`, no terminating event is guaranteed (though
    /// it may happen). Scrolling is in discrete steps and there is no physical
    /// equivalent for the value returned here. For backwards compatibility, the
    /// value returned by this function is identical to a single mouse wheel
    /// rotation by this device (see the documentation for `WheelTilt` above).
    /// Callers should not use this value but instead exclusively refer to the
    //. value returned by `axis_value_discrete`.
    pub fn axis_source(&self) -> AxisSource {
        match unsafe { ffi::libinput_event_pointer_get_axis_source(self.as_raw_mut()) } {
            ffi::libinput_pointer_axis_source_LIBINPUT_POINTER_AXIS_SOURCE_WHEEL => AxisSource::Wheel,
            ffi::libinput_pointer_axis_source_LIBINPUT_POINTER_AXIS_SOURCE_FINGER => AxisSource::Finger,
            ffi::libinput_pointer_axis_source_LIBINPUT_POINTER_AXIS_SOURCE_CONTINUOUS => {
                AxisSource::Continuous
            }
            ffi::libinput_pointer_axis_source_LIBINPUT_POINTER_AXIS_SOURCE_WHEEL_TILT => {
                AxisSource::WheelTilt
            }
            _ => panic!("libinput returned invalid 'libinput_pointer_axis_source'"),
        }
    }

    /// Return the axis value of the given axis.
    ///
    /// The interpretation of the value depends on the axis. For the two scrolling
    /// axes `Vertical` and `Horizontal`, the value of the event is in relative
    /// scroll units, with the positive direction being down or right,
    /// respectively. For the interpretation of the value, see `axis_source`.
    ///
    /// If `has_axis` returns `false` for an axis, this function returns 0 for
    /// that axis.
    pub fn axis_value(&self, axis: Axis) -> f64 {
        unsafe {
            ffi::libinput_event_pointer_get_axis_value(
                self.as_raw_mut(),
                match axis {
                    Axis::Vertical => ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
                    Axis::Horizontal => ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL,
                },
            )
        }
    }

    /// Return the axis value in discrete steps for a given axis event.
    ///
    /// How a value translates into a discrete step depends on the source.
    ///
    /// If the source is `Wheel`, the discrete value correspond to the number of
    /// physical mouse wheel clicks.
    ///
    /// If the source is `Continuous` or `Finger`, the discrete value is always
    /// `None`.
    pub fn axis_value_discrete(&self, axis: Axis) -> Option<f64> {
        match self.axis_source() {
            AxisSource::Continuous | AxisSource::Finger => None,
            _ => Some(unsafe {
                ffi::libinput_event_pointer_get_axis_value_discrete(
                    self.as_raw_mut(),
                    match axis {
                        Axis::Vertical => ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_VERTICAL,
                        Axis::Horizontal => {
                            ffi::libinput_pointer_axis_LIBINPUT_POINTER_AXIS_SCROLL_HORIZONTAL
                        }
                    },
                )
            }),
        }
    }
}
