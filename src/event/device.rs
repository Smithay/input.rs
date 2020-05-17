//! Device event types

use super::EventTrait;
use {AsRaw, Context, FromRaw};
use ffi;

/// Common functions all Device-Events implement.
pub trait DeviceEventTrait: AsRaw<ffi::libinput_event_device_notify> + Context {
    /// Convert into a general `DeviceEvent` again
    fn into_device_event(self) -> DeviceEvent
    where
        Self: Sized,
    {
        unsafe { DeviceEvent::from_raw(self.as_raw_mut(), self.context()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_device_notify> + Context> DeviceEventTrait for T {}

/// A device related `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum DeviceEvent {
    /// Signals that a device has been added to the context.
    Added(DeviceAddedEvent),
    /// Signals that a device has been removed.
    Removed(DeviceRemovedEvent),
}

impl EventTrait for DeviceEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match *self {
            DeviceEvent::Added(ref event) => event.as_raw_event(),
            DeviceEvent::Removed(ref event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_device_notify> for DeviceEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_device_notify, context: &::context::Libinput) -> Self {
        let base = ffi::libinput_event_device_notify_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type_LIBINPUT_EVENT_DEVICE_ADDED => {
                DeviceEvent::Added(DeviceAddedEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_DEVICE_REMOVED => {
                DeviceEvent::Removed(DeviceRemovedEvent::from_raw(event, context))
            }
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_device_notify> for DeviceEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_device_notify {
        match *self {
            DeviceEvent::Added(ref event) => event.as_raw(),
            DeviceEvent::Removed(ref event) => event.as_raw(),
        }
    }
}

impl Context for DeviceEvent {
    fn context(&self) -> &::Libinput {
        match *self {
            DeviceEvent::Added(ref event) => event.context(),
            DeviceEvent::Removed(ref event) => event.context(),
        }
    }
}

ffi_event_struct!(
/// Signals that a device has been added to the context.
///
/// The device will not be read until the next time the user calls
/// `Libinput::dispatch` and data is available.
///
/// This allows setting up initial device configuration before any events are created.
struct DeviceAddedEvent, ffi::libinput_event_device_notify, ffi::libinput_event_device_notify_get_base_event);
ffi_event_struct!(
/// Signals that a device has been removed.
///
/// No more events from the associated device will be in the queue or be queued after this event.
struct DeviceRemovedEvent, ffi::libinput_event_device_notify, ffi::libinput_event_device_notify_get_base_event);
