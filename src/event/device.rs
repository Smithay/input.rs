use ::ffi;

use std::marker::PhantomData;

pub enum DeviceEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Added(DeviceAddedEvent<C, D, G, S, T>),
    Removed(DeviceRemovedEvent<C, D, G, S, T>),
}

pub struct DeviceAddedEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_device_notify,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

pub struct DeviceRemovedEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_device_notify,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}
