use ::ffi;

use std::marker::PhantomData;

pub enum GestureEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Swipe(GestureSwipeEvent<C, D, G, S, T>),
    Pinch(GesturePinchEvent<C, D, G, S, T>),
}

pub enum GestureSwipeEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Begin(GestureSwipeBeginEvent<C, D, G, S, T>),
    Update(GestureSwipeUpdateEvent<C, D, G, S, T>),
    End(GestureSwipeEndEvent<C, D, G, S, T>),
}

pub enum GesturePinchEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Begin(GesturePinchBeginEvent<C, D, G, S, T>),
    Update(GesturePinchUpdateEvent<C, D, G, S, T>),
    End(GesturePinchEndEvent<C, D, G, S, T>),
}

pub struct GestureSwipeBeginEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct GestureSwipeUpdateEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct GestureSwipeEndEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct GesturePinchBeginEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct GesturePinchUpdateEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct GesturePinchEndEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_gesture,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
