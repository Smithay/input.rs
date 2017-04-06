use ::ffi;
use ::{FromRaw, AsRaw};
use super::EventTrait;

use std::marker::PhantomData;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum DeviceEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Added(DeviceAddedEvent<C, D, G, S, T, M>),
    Removed(DeviceRemovedEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_device_notify> for DeviceEvent<C, D, G, S, T, M> {
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

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_device_notify> for DeviceEvent<C, D, G, S, T, M> {
    fn as_raw(&self) -> *const ffi::libinput_event_device_notify {
        match *self {
            DeviceEvent::Added(ref event) => event.as_raw(),
            DeviceEvent::Removed(ref event) => event.as_raw(),
        }
    }
}

ffi_event_struct!(DeviceAddedEvent, ffi::libinput_event_device_notify, ffi::libinput_event_device_notify_get_base_event);
ffi_event_struct!(DeviceRemovedEvent, ffi::libinput_event_device_notify, ffi::libinput_event_device_notify_get_base_event);
