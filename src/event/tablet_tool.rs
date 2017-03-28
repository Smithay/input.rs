use ::ffi;

use std::marker::PhantomData;

pub enum TabletToolEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Axis(TabletToolAxisEvent<C, D, G, S, T>),
    Proximity(TabletToolProximityEvent<C, D, G, S, T>),
    Tip(TabletToolTipEvent<C, D, G, S, T>),
    Button(TabletToolButtonEvent<C, D, G, S, T>),
}

pub struct TabletToolAxisEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TabletToolProximityEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TabletToolTipEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct TabletToolButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
