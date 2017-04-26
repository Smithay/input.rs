//! Touch event types

use super::EventTrait;
use {AsRaw, FromRaw};
use ffi;

/// Common functions all Touch-Events implement.
pub trait TouchEventTrait: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(
    /// The event time for this event
    fn time, ffi::libinput_event_touch_get_time, u32);
    ffi_func!(
    /// The event time for this event in microseconds
    fn time_usec, ffi::libinput_event_touch_get_time_usec, u64);

    /// Convert into a general `TouchEvent` again
    fn into_touch_event(self) -> TouchEvent
        where Self: Sized
    {
        unsafe { TouchEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_touch>> TouchEventTrait for T {}

/// Touch slot related functions all TouchEvents implement, that can be mapped to slots.
///
/// A touch slot is grouping all events related to a single gesture together.
pub trait TouchEventSlot: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(
    /// Get the seat slot of the touch event.
    ///
    /// A seat slot is a non-negative seat wide unique identifier of an active
    /// touch point.
    ///
    /// Events from single touch devices will be represented as one individual
    /// touch point per device.
    fn seat_slot, ffi::libinput_event_touch_get_seat_slot, u32);

    /// Get the slot of this touch event.
    ///
    /// See the kernel's multitouch protocol B documentation for more information.
    ///
    /// If the touch event has no assigned slot, for example if it is from a
    /// single touch device, this function returns `None`.
    fn slot(&self) -> Option<u32> {
        match unsafe { ffi::libinput_event_touch_get_slot(self.as_raw_mut()) } {
            x if x >= 0 => Some(x as u32),
            -1 => None,
            _ => panic!("libinput_event_touch_get_slot returned undocumentated value!"),
        }
    }
}

/// Position related functions all TouchEvents implement, that have a screen
/// position assigned to them.
pub trait TouchEventPosition: AsRaw<ffi::libinput_event_touch> {
    ffi_func!(
    /// Return the current absolute x coordinate of the touch event, in mm from
    /// the top left corner of the device.
    ///
    /// To get the corresponding output screen coordinate, use `x_transformed`.
    fn x, ffi::libinput_event_touch_get_x, f64);
    ffi_func!(
    /// Return the current absolute y coordinate of the touch event, in mm from
    /// the top left corner of the device.
    ///
    /// To get the corresponding output screen coordinate, use `y_transformed`.
    fn y, ffi::libinput_event_touch_get_y, f64);

    /// Return the current absolute x coordinate of the touch event, transformed
    /// to screen coordinates.
    ///
    /// ## Arguments
    ///
    /// - width - The current output screen width
    fn x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_x_transformed(self.as_raw_mut(), width) }
    }

    /// Return the current absolute y coordinate of the touch event, transformed
    /// to screen coordinates.
    ///
    /// ## Arguments
    ///
    /// - height - The current output screen height
    fn y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_touch_get_y_transformed(self.as_raw_mut(), height) }
    }
}

/// A touch related `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TouchEvent {
    /// An event related to resting the finger on the screen
    Down(TouchDownEvent),
    /// An event related to lifting the finger on the screen
    Up(TouchUpEvent),
    /// An event related to moving a finger on the screen
    Motion(TouchMotionEvent),
    /// An event cancelling previous events on this slot
    Cancel(TouchCancelEvent),
    /// Signals the end of a set of touchpoints at one device sample time.
    Frame(TouchFrameEvent),
}

impl EventTrait for TouchEvent {
    #[doc(hidden)]
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
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN => {
                TouchEvent::Down(TouchDownEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP => {
                TouchEvent::Up(TouchUpEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION => {
                TouchEvent::Motion(TouchMotionEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL => {
                TouchEvent::Cancel(TouchCancelEvent::from_raw(event))
            }
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME => {
                TouchEvent::Frame(TouchFrameEvent::from_raw(event))
            }
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

ffi_event_struct!(
/// An event related to resting the finger on the screen
struct TouchDownEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchDownEvent {}

impl TouchEventPosition for TouchDownEvent {}

ffi_event_struct!(
/// An event related to lifting the finger on the screen
struct TouchUpEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchUpEvent {}

ffi_event_struct!(
/// An event related to moving a finger on the screen
struct TouchMotionEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchMotionEvent {}

impl TouchEventPosition for TouchMotionEvent {}

ffi_event_struct!(
/// An event cancelling previous events on this slot
struct TouchCancelEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);

impl TouchEventSlot for TouchCancelEvent {}

ffi_event_struct!(
/// Signals the end of a set of touchpoints at one device sample time.
struct TouchFrameEvent, ffi::libinput_event_touch, ffi::libinput_event_touch_get_base_event);
