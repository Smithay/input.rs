use ::ffi;

use std::marker::PhantomData;

pub enum KeyboardEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Key(KeyboardKeyEvent<C, D, G, S, T>),
}

pub struct KeyboardKeyEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_keyboard,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
