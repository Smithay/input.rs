//! Tablet pad event types

pub use super::{keyboard::KeyState, pointer::ButtonState, EventTrait};
use crate::{ffi, AsRaw, Context, FromRaw, Libinput};

mod mode_group;
pub use self::mode_group::*;

/// Common functions all TabletPad-Events implement.
pub trait TabletPadEventTrait: AsRaw<ffi::libinput_event_tablet_pad> + Context {
    ffi_func!(
    /// The event time for this event
    fn time, ffi::libinput_event_tablet_pad_get_time, u32);
    ffi_func!(
    /// The event time for this event in microseconds
    fn time_usec, ffi::libinput_event_tablet_pad_get_time_usec, u64);
    ffi_func!(
    /// Returns the mode the button, ring, or strip that triggered this event is in, at the time of the event.
    ///
    /// The mode is a virtual grouping of functionality, usually based on some
    /// visual feedback like LEDs on the pad. See [Tablet pad modes](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-pad-modes)
    /// for details. Mode indices start at 0, a device that does not support modes
    /// always returns 0.
    ///
    /// Mode switching is controlled by libinput and more than one mode may exist
    /// on the tablet. This function returns the mode that this event's button,
    /// ring or strip is logically in. If the button is a mode toggle button and
    /// the button event caused a new mode to be toggled, the mode returned is the
    /// new mode the button is in.
    ///
    /// Note that the returned mode is the mode valid as of the time of the event.
    /// The returned mode may thus be different to the mode returned by
    /// `TabletPadModeGroup::mode`. See `TabletPadModeGroup::mode` for details.
    fn mode, ffi::libinput_event_tablet_pad_get_mode, u32);

    /// Returns the mode group that the button, ring, or strip that triggered this
    /// event is considered in.
    ///
    /// The mode is a virtual grouping of functionality, usually based on some
    /// visual feedback like LEDs on the pad. See [Tablet pad modes](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-pad-modes) for details.
    fn mode_group(&self) -> TabletPadModeGroup {
        unsafe {
            TabletPadModeGroup::from_raw(
                ffi::libinput_event_tablet_pad_get_mode_group(self.as_raw_mut()),
                self.context(),
            )
        }
    }

    /// Convert into a general `TabletPadEvent` again
    fn into_tablet_pad_event(self) -> TabletPadEvent
    where
        Self: Sized,
    {
        unsafe { TabletPadEvent::from_raw(self.as_raw_mut(), self.context()) }
    }
}

impl<T: AsRaw<ffi::libinput_event_tablet_pad> + Context> TabletPadEventTrait for T {}

/// A tablet-pad related `Event`
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum TabletPadEvent {
    /// A button pressed on a device with the `DeviceCapability::TabletPad`
    /// capability.
    ///
    /// A Button-Event differs from a `Key`-Event (available with `feature="libinput_1_15"`)
    /// in that buttons are sequentially indexed from 0 and do not carry any other information.
    /// Keys have a specific functionality assigned to them. The key code thus carries a
    /// semantic meaning, a button number does not.
    ///
    /// This event is not to be confused with the button events emitted by tools
    /// on a tablet. See `TabletToolButtonEvent`.
    Button(TabletPadButtonEvent),
    /// A status change on a tablet ring with the `DeviceCapability::TabletPad`
    /// capability.
    Ring(TabletPadRingEvent),
    /// A status change on a strip on a device with the
    /// `DeviceCapability::TabletPad` capability.
    Strip(TabletPadStripEvent),
    /// A key pressed on a device with the `DeviceCapability::TabletPad` capability.
    ///
    /// A `Key`-Event differs from a `Button`-Event in that keys have a specific
    /// functionality assigned to them (buttons are sequencially ordered). A key code
    /// thus carries a semantic meaning, a button number does not.
    #[cfg(feature = "libinput_1_15")]
    Key(TabletPadKeyEvent),
}

impl EventTrait for TabletPadEvent {
    #[doc(hidden)]
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        match self {
            TabletPadEvent::Button(event) => event.as_raw_event(),
            TabletPadEvent::Ring(event) => event.as_raw_event(),
            TabletPadEvent::Strip(event) => event.as_raw_event(),
            #[cfg(feature = "libinput_1_15")]
            TabletPadEvent::Key(event) => event.as_raw_event(),
        }
    }
}

impl FromRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_pad, context: &Libinput) -> Self {
        let base = ffi::libinput_event_tablet_pad_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_BUTTON => {
                TabletPadEvent::Button(TabletPadButtonEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_RING => {
                TabletPadEvent::Ring(TabletPadRingEvent::from_raw(event, context))
            }
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_STRIP => {
                TabletPadEvent::Strip(TabletPadStripEvent::from_raw(event, context))
            }
            #[cfg(feature = "libinput_1_15")]
            ffi::libinput_event_type_LIBINPUT_EVENT_TABLET_PAD_KEY => {
                TabletPadEvent::Key(TabletPadKeyEvent::from_raw(event, context))
            }
            _ => unreachable!(),
        }
    }
}

impl AsRaw<ffi::libinput_event_tablet_pad> for TabletPadEvent {
    fn as_raw(&self) -> *const ffi::libinput_event_tablet_pad {
        match self {
            TabletPadEvent::Button(event) => event.as_raw(),
            TabletPadEvent::Ring(event) => event.as_raw(),
            TabletPadEvent::Strip(event) => event.as_raw(),
            #[cfg(feature = "libinput_1_15")]
            TabletPadEvent::Key(event) => event.as_raw(),
        }
    }
}

impl Context for TabletPadEvent {
    fn context(&self) -> &Libinput {
        match self {
            TabletPadEvent::Button(event) => event.context(),
            TabletPadEvent::Ring(event) => event.context(),
            TabletPadEvent::Strip(event) => event.context(),
            #[cfg(feature = "libinput_1_15")]
            TabletPadEvent::Key(event) => event.context(),
        }
    }
}

ffi_event_struct!(
/// A button pressed on a device with the `DeviceCapability::TabletPad`
/// capability.
///
/// This event is not to be confused with the button events emitted by tools
/// on a tablet. See `TabletToolButtonEvent`.
struct TabletPadButtonEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl TabletPadButtonEvent {
    ffi_func!(
    /// Return the button number that triggered this event, starting at 0.
    ///
    /// Note that the number returned is a generic sequential button number and
    /// not a semantic button code as defined in linux/input.h.
    /// [See Tablet pad button numbers](https://wayland.freedesktop.org/libinput/doc/latest/tablet-support.html#tablet-pad-buttons)
    /// for more details.
    pub fn button_number, ffi::libinput_event_tablet_pad_get_button_number, u32);

    /// Return the button state of the event.
    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_tablet_pad_get_button_state(self.as_raw_mut()) } {
            ffi::libinput_button_state_LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state_LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
            _ => panic!("libinput returned invalid 'libinput_button_state'"),
        }
    }
}

/// The source for a `TabletPadRingEvent` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum RingAxisSource {
    /// An unknown source
    Unknown,
    /// Finger source
    Finger,
}

ffi_event_struct!(
/// A status change on a tablet ring with the `DeviceCapability::TabletPad`
/// capability.
struct TabletPadRingEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl TabletPadRingEvent {
    ffi_func!(
    /// Returns the number of the ring that has changed state, with 0 being the
    /// first ring.
    ///
    /// On tablets with only one ring, this function always returns 0.
    pub fn number, ffi::libinput_event_tablet_pad_get_ring_number, u32);
    ffi_func!(
    /// Returns the current position of the ring, in degrees counterclockwise from
    /// the northern-most point of the ring in the tablet's current logical
    /// orientation.
    ///
    /// If the source is `RingAxisSource::Finger`, libinput sends a  terminating
    /// event with a ring value of -1 when the finger is lifted from the ring. A
    /// caller may use this information to e.g. determine if kinetic scrolling
    /// should be triggered.
    pub fn position, ffi::libinput_event_tablet_pad_get_ring_position, f64);

    /// Returns the source of the interaction with the ring.
    ///
    /// If the source is `RingAxisSource::Finger`, libinput sends a ring position
    /// value of -1 to terminate the current interaction.
    pub fn source(&self) -> RingAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_ring_source(self.as_raw_mut()) } {
            ffi::libinput_tablet_pad_ring_axis_source_LIBINPUT_TABLET_PAD_RING_SOURCE_UNKNOWN => {
                RingAxisSource::Unknown
            }
            ffi::libinput_tablet_pad_ring_axis_source_LIBINPUT_TABLET_PAD_RING_SOURCE_FINGER => {
                RingAxisSource::Finger
            }
            _ => panic!("libinput returned invalid 'libinput_tablet_pad_ring_axis_source'"),
        }
    }
}

/// The source for a `TabletPadStripEvent` event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StripAxisSource {
    /// An unknown source
    Unknown,
    /// Finger source
    Finger,
}

ffi_event_struct!(
/// A status change on a strip on a device with the `DeviceCapability::TabletPad`
/// capability.
struct TabletPadStripEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

impl TabletPadStripEvent {
    ffi_func!(
    /// Returns the number of the strip that has changed state, with 0 being the
    /// first strip.
    ///
    /// On tablets with only one strip, this function always returns 0.
    pub fn number, ffi::libinput_event_tablet_pad_get_strip_number, u32);
    ffi_func!(
    /// Returns the current position of the strip, normalized to the range [0, 1],
    /// with 0 being the top/left-most point in the tablet's current logical
    /// orientation.
    ///
    /// If the source is `StripAxisSource::Finger`, libinput sends a terminating
    /// event with a ring value of -1 when the finger is lifted from the ring. A
    /// caller may use this information to e.g. determine if kinetic scrolling
    /// should be triggered.
    pub fn position, ffi::libinput_event_tablet_pad_get_strip_position, f64);

    /// Returns the source of the interaction with the strip.
    ///
    /// If the source is `StripAxisSource::Finger`, libinput sends a strip
    /// position value of -1 to terminate the current interaction
    pub fn source(&self) -> StripAxisSource {
        match unsafe { ffi::libinput_event_tablet_pad_get_strip_source(self.as_raw_mut()) } {
            ffi::libinput_tablet_pad_strip_axis_source_LIBINPUT_TABLET_PAD_STRIP_SOURCE_UNKNOWN => {
                StripAxisSource::Unknown
            }
            ffi::libinput_tablet_pad_strip_axis_source_LIBINPUT_TABLET_PAD_STRIP_SOURCE_FINGER => {
                StripAxisSource::Finger
            }
            _ => panic!("libinput returned invalid 'libinput_tablet_pad_strip_axis_source'"),
        }
    }
}

#[cfg(feature = "libinput_1_15")]
ffi_event_struct!(
/// A key pressed on a device with the `DeviceCapability::TabletPad` capability.
struct TabletPadKeyEvent, ffi::libinput_event_tablet_pad, ffi::libinput_event_tablet_pad_get_base_event);

#[cfg(feature = "libinput_1_15")]
impl TabletPadKeyEvent {
    ffi_func!(
    /// Return the key code that triggered this event, e.g. KEY_CONTROLPANEL.
    /// The list of key codes is defined in linux/input-event-codes.h
    pub fn key, ffi::libinput_event_tablet_pad_get_key, u32);

    /// Return the key state of the event
    pub fn key_state(&self) -> KeyState {
        match unsafe { ffi::libinput_event_tablet_pad_get_key_state(self.as_raw() as *mut _) } {
            ffi::libinput_key_state_LIBINPUT_KEY_STATE_PRESSED => KeyState::Pressed,
            ffi::libinput_key_state_LIBINPUT_KEY_STATE_RELEASED => KeyState::Released,
            _ => panic!("libinput returned invalid 'libinput_key_state'"),
        }
    }
}
