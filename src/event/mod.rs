use ::{ffi, Libinput, Device, FromRaw, AsRaw};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Device(DeviceEvent<C, D, G, S, T, M>),
    Keyboard(KeyboardEvent<C, D, G, S, T, M>),
    Pointer(PointerEvent<C, D, G, S, T, M>),
    Touch(TouchEvent<C, D, G, S, T, M>),
    Tablet(TabletToolEvent<C, D, G, S, T, M>),
    TabletPad(TabletPadEvent<C, D, G, S, T, M>),
    Gesture(GestureEvent<C, D, G, S, T, M>),
    Switch(SwitchEvent<C, D, G, S, T, M>),
}

pub trait EventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event;

    fn into_event(self) -> Event<C, D, G, S, T, M> where Self: Sized {
        unsafe { Event::from_raw(self.as_raw_event()) }
    }

    fn context(&self) -> Libinput<C, D, G, S, T, M> {
        unsafe {
            Libinput::from_raw(ffi::libinput_event_get_context(self.as_raw_event()))
        }
    }

    fn device(&self) -> Device<C, D, G, S, T, M> {
        unsafe {
            Device::from_raw(ffi::libinput_event_get_device(self.as_raw_event()))
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> EventTrait<C, D, G, S, T, M> for Event<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { self.as_raw_mut() }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event> for Event<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event) -> Self {
        match ffi::libinput_event_get_type(event) {
            ffi::libinput_event_type::LIBINPUT_EVENT_NONE => panic!("Trying to convert null event"),
            ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_ADDED | ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_REMOVED => Event::Device(DeviceEvent::from_raw(ffi::libinput_event_get_device_notify_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY => Event::Keyboard(KeyboardEvent::from_raw(ffi::libinput_event_get_keyboard_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_MOTION_ABSOLUTE | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_BUTTON | ffi::libinput_event_type::LIBINPUT_EVENT_POINTER_AXIS => Event::Pointer(PointerEvent::from_raw(ffi::libinput_event_get_pointer_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_DOWN | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_UP | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_MOTION | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_CANCEL | ffi::libinput_event_type::LIBINPUT_EVENT_TOUCH_FRAME => Event::Touch(TouchEvent::from_raw(ffi::libinput_event_get_touch_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_AXIS | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_TIP | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_BUTTON => Event::Tablet(TabletToolEvent::from_raw(ffi::libinput_event_get_tablet_tool_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_BUTTON | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_RING | ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_PAD_STRIP => Event::TabletPad(TabletPadEvent::from_raw(ffi::libinput_event_get_tablet_pad_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_SWIPE_END | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_BEGIN | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_UPDATE | ffi::libinput_event_type::LIBINPUT_EVENT_GESTURE_PINCH_END => Event::Gesture(GestureEvent::from_raw(ffi::libinput_event_get_gesture_event(event))),
            ffi::libinput_event_type::LIBINPUT_EVENT_SWITCH_TOGGLE => Event::Switch(SwitchEvent::from_raw(ffi::libinput_event_get_switch_event(event))),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event> for Event<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event {
        match *self {
            Event::Device(ref event) => event.as_raw_event() as *const _,
            Event::Keyboard(ref event) => event.as_raw_event() as *const _,
            Event::Pointer(ref event) => event.as_raw_event() as *const _,
            Event::Touch(ref event) => event.as_raw_event() as *const _,
            Event::Tablet(ref event) => event.as_raw_event() as *const _,
            Event::TabletPad(ref event) => event.as_raw_event() as *const _,
            Event::Gesture(ref event) => event.as_raw_event() as *const _,
            Event::Switch(ref event) => event.as_raw_event() as *const _,
        }
    }
}

macro_rules! ffi_event_struct {
    ($struct_name:ident, $ffi_name:path, $get_base_fn:path) => (
        ffi_struct!($struct_name, $ffi_name);

        impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> EventTrait<C, D, G, S, T, M> for $struct_name<C, D, G, S, T, M> {
            fn as_raw_event(&self) -> *mut ffi::libinput_event {
                unsafe { $get_base_fn(self.as_raw_mut()) }
            }
        }
    )
}

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
