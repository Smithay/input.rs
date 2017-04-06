use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

use std::marker::PhantomData;

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
}

impl<T: AsRaw<ffi::libinput_event_keyboard>> KeyboardEventTrait for T {}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum KeyboardEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Key(KeyboardKeyEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_keyboard> for KeyboardEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_keyboard) -> Self {
        let base = ffi::libinput_event_keyboard_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_KEYBOARD_KEY =>
                KeyboardEvent::Key(KeyboardKeyEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_keyboard> for KeyboardEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_keyboard {
        match *self {
            KeyboardEvent::Key(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(KeyboardKeyEvent, ffi::libinput_event_keyboard, ffi::libinput_event_keyboard_get_base_event);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> KeyboardKeyEvent<C, D, G, S, T, M> {
    ffi_func!(pub seat_key_count, ffi::libinput_event_keyboard_get_seat_key_count, u32);
}
