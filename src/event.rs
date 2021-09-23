//! Libinput Events

use crate::{ffi, AsRaw, Context, Device, FromRaw, Libinput};

/// A libinput `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Event {
    /// A device related `Event`
    Device(DeviceEvent),
    /// A keyboard related `Event`
    Keyboard(KeyboardEvent),
    /// A pointer related `Event`
    Pointer(PointerEvent),
    /// A touch related `Event`
    Touch(TouchEvent),
    /// A tablet related `Event`
    Tablet(TabletToolEvent),
    /// A tabled pad related `Event`
    TabletPad(TabletPadEvent),
    /// A gesture related `Event`
    Gesture(GestureEvent),
    /// A switch related `Event`
    Switch(SwitchEvent),
}

/// Common functions all (Sub-)Events implement.
pub trait EventTrait: Context {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event;

    /// Convert into a general `Event` again
    fn into_event(self) -> Event
    where
        Self: Sized,
    {
        unsafe { Event::from_raw(self.as_raw_event(), self.context()) }
    }

    /// Return the device associated with this event.
    ///
    /// For device added/removed events this is the device added or removed.
    /// For all other device events, this is the device that generated the event.
    fn device(&self) -> Device {
        unsafe {
            Device::from_raw(
                ffi::libinput_event_get_device(self.as_raw_event()),
                self.context(),
            )
        }
    }
}

impl EventTrait for Event {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        self.as_raw_mut()
    }
}

impl FromRaw<ffi::libinput_event> for Event {
    unsafe fn from_raw(event: *mut ffi::libinput_event, context: &Libinput) -> Self {
        match ffi::libinput_event_get_type(event) {
            ffi::libinput_event_type_LIBINPUT_EVENT_NONE => panic!("Trying to convert null event"),
            ffi::libinput_event_type_LIBINPUT_EVENT_DEVICE_ADDED
            | ffi::libinput_event_type_LIBINPUT_EVENT_DEVICE_REMOVED => Event::Device(
                DeviceEvent::from_raw(ffi::libinput_event_get_device_notify_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_KEYBOARD_KEY => Event::Keyboard(
                KeyboardEvent::from_raw(ffi::libinput_event_get_keyboard_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION
            | ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE
            | ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_BUTTON
            | ffi::libinput_event_type_LIBINPUT_EVENT_POINTER_AXIS => Event::Pointer(
                PointerEvent::from_raw(ffi::libinput_event_get_pointer_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_TOUCH_DOWN
            | ffi::libinput_event_type_LIBINPUT_EVENT_TOUCH_UP
            | ffi::libinput_event_type_LIBINPUT_EVENT_TOUCH_MOTION
            | ffi::libinput_event_type_LIBINPUT_EVENT_TOUCH_CANCEL
            | ffi::libinput_event_type_LIBINPUT_EVENT_TOUCH_FRAME => Event::Touch(
                TouchEvent::from_raw(ffi::libinput_event_get_touch_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_AXIS
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_TIP
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_TOOL_BUTTON => {
                Event::Tablet(TabletToolEvent::from_raw(
                    ffi::libinput_event_get_tablet_tool_event(event),
                    context,
                ))
            }
            #[cfg(not(feature = "libinput_1_15"))]
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_BUTTON
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_RING
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_STRIP => Event::TabletPad(
                TabletPadEvent::from_raw(ffi::libinput_event_get_tablet_pad_event(event), context),
            ),
            #[cfg(feature = "libinput_1_15")]
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_BUTTON
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_RING
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_STRIP
            | ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_KEY => Event::TabletPad(
                TabletPadEvent::from_raw(ffi::libinput_event_get_tablet_pad_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN
            | ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE
            | ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_SWIPE_END
            | ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_BEGIN
            | ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_UPDATE
            | ffi::libinput_event_type_LIBINPUT_EVENT_GESTURE_PINCH_END => Event::Gesture(
                GestureEvent::from_raw(ffi::libinput_event_get_gesture_event(event), context),
            ),
            ffi::libinput_event_type_LIBINPUT_EVENT_SWITCH_TOGGLE => Event::Switch(
                SwitchEvent::from_raw(ffi::libinput_event_get_switch_event(event), context),
            ),
            _ => panic!("libinput returned invalid 'libinput_event_type'"),
        }
    }
}

impl AsRaw<ffi::libinput_event> for Event {
    fn as_raw(&self) -> *const ffi::libinput_event {
        match self {
            Event::Device(event) => event.as_raw_event() as *const _,
            Event::Keyboard(event) => event.as_raw_event() as *const _,
            Event::Pointer(event) => event.as_raw_event() as *const _,
            Event::Touch(event) => event.as_raw_event() as *const _,
            Event::Tablet(event) => event.as_raw_event() as *const _,
            Event::TabletPad(event) => event.as_raw_event() as *const _,
            Event::Gesture(event) => event.as_raw_event() as *const _,
            Event::Switch(event) => event.as_raw_event() as *const _,
        }
    }
}

impl Context for Event {
    fn context(&self) -> &crate::Libinput {
        match self {
            Event::Device(event) => event.context(),
            Event::Keyboard(event) => event.context(),
            Event::Pointer(event) => event.context(),
            Event::Touch(event) => event.context(),
            Event::Tablet(event) => event.context(),
            Event::TabletPad(event) => event.context(),
            Event::Gesture(event) => event.context(),
            Event::Switch(event) => event.context(),
        }
    }
}

macro_rules! ffi_event_struct {
    ($(#[$attr:meta])* struct $struct_name:ident, $ffi_name:path, $get_base_fn:path) => (
        #[derive(Eq)]
        $(#[$attr])*
        pub struct $struct_name
        {
            ffi: *mut $ffi_name,
            context: $crate::context::Libinput,
        }

        impl std::fmt::Debug for $struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                write!(f, "$struct_name @{:p}", self.as_raw())
            }
        }

        impl FromRaw<$ffi_name> for $struct_name
        {
            unsafe fn from_raw(ffi: *mut $ffi_name, context: &$crate::context::Libinput) -> Self {
                $struct_name {
                    ffi,
                    context: context.clone(),
                }
            }
        }

        impl AsRaw<$ffi_name> for $struct_name
        {
            fn as_raw(&self) -> *const $ffi_name {
                self.ffi as *const _
            }
        }

        impl $crate::Context for $struct_name {
            fn context(&self) -> &$crate::context::Libinput {
                &self.context
            }
        }

        impl PartialEq for $struct_name {
            fn eq(&self, other: &Self) -> bool {
                self.as_raw() == other.as_raw()
            }
        }

        impl std::hash::Hash for $struct_name {
            fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
                self.as_raw().hash(state);
            }
        }

        impl EventTrait for $struct_name {
            #[doc(hidden)]
            fn as_raw_event(&self) -> *mut $crate::ffi::libinput_event {
                unsafe { $get_base_fn(self.as_raw_mut()) }
            }
        }

        impl Drop for $struct_name {
            fn drop(&mut self) {
                unsafe { $crate::ffi::libinput_event_destroy(self.as_raw_event()) }
            }
        }
    )
}

pub mod device;
pub mod gesture;
pub mod keyboard;
pub mod pointer;
pub mod switch;
pub mod tablet_pad;
pub mod tablet_tool;
pub mod touch;

pub use self::device::DeviceEvent;
pub use self::gesture::GestureEvent;
pub use self::keyboard::KeyboardEvent;
pub use self::pointer::PointerEvent;
pub use self::switch::SwitchEvent;
pub use self::tablet_pad::TabletPadEvent;
pub use self::tablet_tool::TabletToolEvent;
pub use self::touch::TouchEvent;
