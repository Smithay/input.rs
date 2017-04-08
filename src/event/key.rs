use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KeyState {
    Pressed,
    Released,
}

pub trait KeyboardEventTrait: AsRaw<ffi::libinput_event_keyboard> {
    ffi_func!(time, ffi::libinput_event_keyboard_get_time, u32);
    ffi_func!(time_usec, ffi::libinput_event_keyboard_get_time_usec, u64);
    ffi_func!(key, ffi::libinput_event_keyboard_get_key, u32);

    fn key_state(&self) -> KeyState {
        match unsafe { ffi::libinput_event_keyboard_get_key_state(self.as_raw() as *mut _) } {
            ffi::libinput_key_state::LIBINPUT_KEY_STATE_PRESSED => KeyState::Pressed,
            ffi::libinput_key_state::LIBINPUT_KEY_STATE_RELEASED => KeyState::Released,
        }
    }

    fn into_keyboard_event(self) -> KeyboardEvent where Self: Sized {
        unsafe { KeyboardEvent::from_raw(self.as_raw_mut()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_keyboard>> KeyboardEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardEvent {
    Key(KeyboardKeyEvent),
}

impl EventTrait for KeyboardEvent {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            KeyboardEvent::Key(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_keyboard> for KeyboardEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_keyboard) -> Self {
        let base = ffi::libinput_event_keyboard_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY =>
                KeyboardEvent::Key(KeyboardKeyEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_keyboard> for KeyboardEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_keyboard {
        match *self {
            KeyboardEvent::Key(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(KeyboardKeyEvent, ffi::libinput_event_keyboard, ffi::libinput_event_keyboard_get_base_event);

impl KeyboardKeyEvent {
    ffi_func!(pub seat_key_count, ffi::libinput_event_keyboard_get_seat_key_count, u32);
}
