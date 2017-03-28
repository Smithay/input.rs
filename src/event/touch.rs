use ::ffi;

use std::marker::PhantomData;

pub enum TouchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Down(TouchDownEvent<C, D, G, S, T>),
    Up(TouchUpEvent<C, D, G, S, T>),
    Motion(TouchMotionEvent<C, D, G, S, T>),
    Cancel(TouchCancelEvent<C, D, G, S, T>),
    Frame(TouchFrameEvent<C, D, G, S, T>),
}

pub struct TouchDownEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TouchUpEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TouchMotionEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TouchCancelEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TouchFrameEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_touch,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
