use ::ffi;
use ::{FromRaw, AsRaw, ButtonState};
use super::Event;

use std::marker::PhantomData;

mod tool;
pub use self::tool::TabletTool;

pub trait TabletToolEventTrait<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>: AsRaw<ffi::libinput_event_tablet_tool>
{
    fn time(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_tool_get_time(self.as_raw() as *mut _) }
    }

    fn time_usec(&self) -> u64 {
        unsafe { ffi::libinput_event_tablet_tool_get_time_usec(self.as_raw() as *mut _) }
    }

    fn distance_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_distance_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn distance(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_distance(self.as_raw() as *mut _) }
    }

    fn dx(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_dx(self.as_raw() as *mut _) }
    }

    fn dy(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_dy(self.as_raw() as *mut _) }
    }

    fn pressure_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_pressure_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn pressure(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_pressure(self.as_raw() as *mut _) }
    }

    fn rotation_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_rotation_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn rotation(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_rotation(self.as_raw() as *mut _) }
    }

    fn slider_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_slider_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn slider_position(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_slider_position(self.as_raw() as *mut _) }
    }

    fn tilt_x_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_tilt_x_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn tilt_x(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_tilt_x(self.as_raw() as *mut _) }
    }

    fn tilt_y_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_tilt_y_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn tilt_y(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_tilt_y(self.as_raw() as *mut _) }
    }

    fn wheel_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_wheel_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn wheel_delta(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_wheel_delta(self.as_raw() as *mut _) }
    }

    fn wheel_delta_discrete(&self) -> i32 {
        unsafe { ffi::libinput_event_tablet_tool_get_wheel_delta_discrete(self.as_raw() as *mut _) }
    }

    fn x_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_x_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn x(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_x(self.as_raw() as *mut _) }
    }

    fn y_has_changed(&self) -> bool {
        unsafe { ffi::libinput_event_tablet_tool_y_has_changed(self.as_raw() as *mut _) != 0 }
    }

    fn y(&self) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_y(self.as_raw() as *mut _) }
    }

    fn x_transformed(&self, width: u32) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_x_transformed(self.as_raw() as *mut _, width) }
    }

    fn y_transformed(&self, height: u32) -> f64 {
        unsafe { ffi::libinput_event_tablet_tool_get_x_transformed(self.as_raw() as *mut _, height) }
    }

    fn tool(&self) -> TabletTool<C, D, G, S, T, M> {
        unsafe { TabletTool::from_raw(ffi::libinput_event_tablet_tool_get_tool(self.as_raw() as *mut _)) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static, R: AsRaw<ffi::libinput_event_tablet_tool>> TabletToolEventTrait<C, D, G, S, T, M> for R {}

#[derive(Clone, Copy)]
pub enum TabletToolEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    Axis(TabletToolAxisEvent<C, D, G, S, T, M>),
    Proximity(TabletToolProximityEvent<C, D, G, S, T, M>),
    Tip(TabletToolTipEvent<C, D, G, S, T, M>),
    Button(TabletToolButtonEvent<C, D, G, S, T, M>),
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletToolEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_tool_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_tool> for TabletToolEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        let base = ffi::libinput_event_tablet_tool_get_base_event(event);
        match ffi::libinput_event_get_type(base) {
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_AXIS =>
                TabletToolEvent::Axis(TabletToolAxisEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_PROXIMITY =>
                TabletToolEvent::Proximity(TabletToolProximityEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_TIP =>
                TabletToolEvent::Tip(TabletToolTipEvent::from_raw(event)),
            ffi::libinput_event_type::LIBINPUT_EVENT_TABLET_TOOL_BUTTON =>
                TabletToolEvent::Button(TabletToolButtonEvent::from_raw(event)),
            _ => unreachable!(),
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_tool> for TabletToolEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        match *self {
            TabletToolEvent::Axis(ref event) => event.as_raw(),
            TabletToolEvent::Proximity(ref event) => event.as_raw(),
            TabletToolEvent::Tip(ref event) => event.as_raw(),
            TabletToolEvent::Button(ref event) => event.as_raw(),
        }
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_tool {
        match *self {
            TabletToolEvent::Axis(ref mut event) => event.as_raw_mut(),
            TabletToolEvent::Proximity(ref mut event) => event.as_raw_mut(),
            TabletToolEvent::Tip(ref mut event) => event.as_raw_mut(),
            TabletToolEvent::Button(ref mut event) => event.as_raw_mut(),
        }
    }
}

#[derive(Clone, Copy)]
pub struct TabletToolAxisEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletToolAxisEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_tool_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_tool> for TabletToolAxisEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        TabletToolAxisEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_tool> for TabletToolAxisEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_tool {
        self.event
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ProximityState {
    Out,
    In,
}

#[derive(Clone, Copy)]
pub struct TabletToolProximityEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletToolProximityEvent<C, D, G, S, T, M> {
    pub fn proximity_state(&self) -> ProximityState {
        match unsafe { ffi::libinput_event_tablet_tool_get_proximity_state(self.as_raw() as *mut _) } {
            ffi::libinput_tablet_tool_proximity_state::LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_OUT => ProximityState::Out,
            ffi::libinput_tablet_tool_proximity_state::LIBINPUT_TABLET_TOOL_PROXIMITY_STATE_IN => ProximityState::In,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletToolProximityEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_tool_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_tool> for TabletToolProximityEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        TabletToolProximityEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_tool> for TabletToolProximityEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_tool {
        self.event
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TipState {
    Up,
    Down,
}

#[derive(Clone, Copy)]
pub struct TabletToolTipEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletToolTipEvent<C, D, G, S, T, M> {
    pub fn tip_state(&self) -> TipState {
        match unsafe { ffi::libinput_event_tablet_tool_get_tip_state(self.as_raw() as *mut _) } {
            ffi::libinput_tablet_tool_tip_state::LIBINPUT_TABLET_TOOL_TIP_UP => TipState::Up,
            ffi::libinput_tablet_tool_tip_state::LIBINPUT_TABLET_TOOL_TIP_DOWN => TipState::Down,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletToolTipEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_tool_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_tool> for TabletToolTipEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        TabletToolTipEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_tool> for TabletToolTipEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_tool {
        self.event
    }
}

#[derive(Clone, Copy)]
pub struct TabletToolButtonEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    event: *mut ffi::libinput_event_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletToolButtonEvent<C, D, G, S, T, M> {
    pub fn button(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_tool_get_button(self.as_raw() as *mut _) }
    }

    pub fn button_state(&self) -> ButtonState {
        match unsafe { ffi::libinput_event_tablet_tool_get_button_state(self.as_raw() as *mut _) } {
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_PRESSED => ButtonState::Pressed,
            ffi::libinput_button_state::LIBINPUT_BUTTON_STATE_RELEASED => ButtonState::Released,
        }
    }

    pub fn seat_button_count(&self) -> u32 {
        unsafe { ffi::libinput_event_tablet_tool_get_seat_button_count(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Event<C, D, G, S, T, M> for TabletToolButtonEvent<C, D, G, S, T, M> {
    fn as_raw_event(&self) -> *mut ffi::libinput_event {
        unsafe { ffi::libinput_event_tablet_tool_get_base_event(self.as_raw() as *mut _) }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> FromRaw<ffi::libinput_event_tablet_tool> for TabletToolButtonEvent<C, D, G, S, T, M> {
    unsafe fn from_raw(event: *mut ffi::libinput_event_tablet_tool) -> Self {
        TabletToolButtonEvent {
            event: event,
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> AsRaw<ffi::libinput_event_tablet_tool> for TabletToolButtonEvent<C, D, G, S, T, M> {
    unsafe fn as_raw(&self) -> *const ffi::libinput_event_tablet_tool {
        self.event as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_event_tablet_tool {
        self.event
    }
}
