mod device;
mod gesture;
mod key;
mod pointer;
mod switch;
mod tablet_tool;
mod tablet_pad;
mod touch;

pub use self::device::*;
pub use self::gesture::*;
pub use self::key::*;
pub use self::pointer::*;
pub use self::switch::*;
pub use self::tablet_tool::*;
pub use self::tablet_pad::*;
pub use self::touch::*;

use ::{ffi, LibinputContext, LibinputDevice, FromRaw, AsRaw};

#[derive(Clone, Copy)]
pub enum LibinputEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Device(DeviceEvent<C, D, G, S, T>),
    Keyboard(KeyboardEvent<C, D, G, S, T>),
    Pointer(PointerEvent<C, D, G, S, T>),
    Touch(TouchEvent<C, D, G, S, T>),
    Tablet(TabletToolEvent<C, D, G, S, T>),
    TabletPad(TabletPadEvent<C, D, G, S, T>),
    Gesture(GestureEvent<C, D, G, S, T>),
    Switch(SwitchEvent<C, D, G, S, T>),
}

pub trait Event<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event;

    fn context(&self) -> LibinputContext<C, D, G, S, T> {
        unsafe {
            LibinputContext::from_raw(ffi::libinput_event_get_context(self.as_raw_event()))
        }
    }

    fn device(&self) -> LibinputDevice<C, D, G, S, T> {
        unsafe {
            LibinputDevice::from_raw(ffi::libinput_event_get_device(self.as_raw_event()))
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for LibinputEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { self.as_raw() as *mut _ }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event> for LibinputEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event) -> Self {
        match ffi::libinput_event_get_type(event) {
            ffi::libinput_event_type::LIBINPUT_EVENT_NONE => panic!("Trying to convert null event"),
            ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_ADDED | ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_REMOVED => LibinputEvent::Device(DeviceEvent::from_raw(ffi::libinput_event_get_device_notify_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY => LibinputEvent::Keyboard(KeyboardEvent::from_raw(ffi::libinput_event_get_keyboard_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_BUTTON | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_AXIS => LibinputEvent::Pointer(PointerEvent::from_raw(ffi::libinput_event_get_pointer_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME => LibinputEvent::Touch(TouchEvent::from_raw(ffi::libinput_event_get_touch_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_AXIS | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_TIP | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_BUTTON => LibinputEvent::Tablet(TabletToolEvent::from_raw(ffi::libinput_event_get_tablet_tool_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_BUTTON | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_RING | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_STRIP => LibinputEvent::TabletPad(TabletPadEvent::from_raw(ffi::libinput_event_get_tablet_pad_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => LibinputEvent::Gesture(GestureEvent::from_raw(ffi::libinput_event_get_gesture_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE => LibinputEvent::Switch(SwitchEvent::from_raw(ffi::libinput_event_get_switch_event(event))),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event> for LibinputEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event {
        match *self {
            LibinputEvent::Device(ref event) => ffi::libinput_event_device_notify_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Keyboard(ref event) => ffi::libinput_event_keyboard_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Pointer(ref event) => ffi::libinput_event_pointer_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Touch(ref event) => ffi::libinput_event_touch_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Tablet(ref event) => ffi::libinput_event_tablet_tool_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::TabletPad(ref event) => ffi::libinput_event_tablet_pad_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Gesture(ref event) => ffi::libinput_event_gesture_get_base_event(event.as_raw() as *mut _) as *const _,
            LibinputEvent::Switch(ref event) => ffi::libinput_event_switch_get_base_event(event.as_raw() as *mut _) as *const _,
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event {
        match *self {
            LibinputEvent::Device(ref mut event) => ffi::libinput_event_device_notify_get_base_event(event.as_raw_mut()),
            LibinputEvent::Keyboard(ref mut event) => ffi::libinput_event_keyboard_get_base_event(event.as_raw_mut()),
            LibinputEvent::Pointer(ref mut event) => ffi::libinput_event_pointer_get_base_event(event.as_raw_mut()),
            LibinputEvent::Touch(ref mut event) => ffi::libinput_event_touch_get_base_event(event.as_raw_mut()),
            LibinputEvent::Tablet(ref mut event) => ffi::libinput_event_tablet_tool_get_base_event(event.as_raw_mut()),
            LibinputEvent::TabletPad(ref mut event) => ffi::libinput_event_tablet_pad_get_base_event(event.as_raw_mut()),
            LibinputEvent::Gesture(ref mut event) => ffi::libinput_event_gesture_get_base_event(event.as_raw_mut()),
            LibinputEvent::Switch(ref mut event) => ffi::libinput_event_switch_get_base_event(event.as_raw_mut()),
        }
    }
}
