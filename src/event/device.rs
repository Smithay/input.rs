use ::ffi;
use ::{FromRaw, AsRaw};
use super::Event;

use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub enum DeviceEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Added(DeviceAddedEvent<C, D, G, S, T>),
    Removed(DeviceRemovedEvent<C, D, G, S, T>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for DeviceEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_device_notify_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_device_notify> for DeviceEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_device_notify) -> Self {
        let base = ffi::libinput_event_device_notify_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_ADDED =>
                DeviceEvent::Added(DeviceAddedEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_DEVICE_REMOVED =>
                DeviceEvent::Removed(DeviceRemovedEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_device_notify> for DeviceEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_device_notify {
        match *self {
            DeviceEvent::Added(ref event) => event.as_raw(),
            DeviceEvent::Removed(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_device_notify {
        match *self {
            DeviceEvent::Added(ref mut event) => event.as_raw_mut(),
            DeviceEvent::Removed(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct DeviceAddedEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_device_notify,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for DeviceAddedEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_device_notify_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_device_notify> for DeviceAddedEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_device_notify) -> Self {
        DeviceAddedEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_device_notify> for DeviceAddedEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_device_notify {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_device_notify {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct DeviceRemovedEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    event: *mut ffi::libinput_event_device_notify,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> Event<C, D, G, S, T> for DeviceRemovedEvent<C, D, G, S, T> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_device_notify_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> FromRaw<ffi::libinput_event_device_notify> for DeviceRemovedEvent<C, D, G, S, T> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_device_notify) -> Self {
        DeviceRemovedEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> AsRaw<ffi::libinput_event_device_notify> for DeviceRemovedEvent<C, D, G, S, T> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_device_notify {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_device_notify {
        self.event
    }
}
