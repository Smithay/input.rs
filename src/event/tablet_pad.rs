use ::ffi;

use std::marker::PhantomData;

pub enum TabletPadEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Button(TabletPadButtonEvent<C, D, G, S, T>),
    Ring(TabletPadRingEvent<C, D, G, S, T>),
    Strip(TabletPadStripEvent<C, D, G, S, T>),
}

pub struct TabletPadButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TabletPadRingEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static>  {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TabletPadStripEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_pad,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
