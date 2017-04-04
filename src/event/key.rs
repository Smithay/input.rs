use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

#[derive(Debug, Clone, Copy)]
pub enum KeyState {
    Pressed,
    Released,
}

pub trait KeyboardEventTrait: AsRaw<ffi::libinput_event_keyboard> {
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_keyboard_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_keyboard_get_time_usec(self.as_raw() as *mut _) }
    }

    fn key(&self) -> u32 {
        unsafe { ffi::libinput_event_keyboard_get_key(self.as_raw() as *mut _) }
    }

    fn key_state(&self) -> KeyState {
        match unsafe { ffi::libinput_event_keyboard_get_key_state(self.as_raw() as *mut _) } {
            ffi::libinput_key_state::LIBINPUT_KEY_STATE_PRESSED => KeyState::Pressed,
            ffi::libinput_key_state::LIBINPUT_KEY_STATE_RELEASED => KeyState::Released,
        }
    }
}

impl<T: AsRaw<ffi::libinput_event_keyboard>> KeyboardEventTrait for T {}

#[derive(Clone, Copy)]
pub enum KeyboardEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Key(KeyboardKeyEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for KeyboardEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_keyboard_get_base_event(self.as_raw() as *mut _) }
    }
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
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_keyboard {
        match *self {
            KeyboardEvent::Key(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_keyboard {
        match *self {
            KeyboardEvent::Key(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct KeyboardKeyEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_keyboard,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> KeyboardKeyEvent<C, D, G, S, T, M> {
    pub fn seat_key_count(&self) -> u32 {
        unsafe { ffi::libinput_event_keyboard_get_seat_key_count(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for KeyboardKeyEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_keyboard_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_keyboard> for KeyboardKeyEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_keyboard) -> Self {
        KeyboardKeyEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_keyboard> for KeyboardKeyEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_keyboard {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_keyboard {
        self.event
    }
}
