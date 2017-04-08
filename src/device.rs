use std::ffi::{CStr, CString};

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata, Libinput, Seat, TabletPadModeGroup};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceCapability
{
    Keyboard,
    Pointer,
    Touch,
    TabletTool,
    TabletPad,
    Gesture,
    Switch,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum AccelProfile
{
    Flat,
    Adaptive,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ClickMethod
{
    ButtonAreas,
    Clickfinger,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ScrollMethod
{
    NoScroll,
    TwoFinger,
    Edge,
    OnButtonDown,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DeviceConfigError {
    Unsupported,
    Invalid,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum SendEventsMode {
    Enabled,
    Disabled,
    DisabledOnExternalMouse,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum TapButtonMap {
    LeftRightMiddle,
    LeftMiddleRight,
}

pub type DeviceConfigResult = Result<(), DeviceConfigError>;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Led {
    NumLock,
    CapsLock,
    ScrollLock,
}

ffi_ref_struct!(struct DeviceGroup, ffi::libinput_device_group, ffi::libinput_device_group_ref, ffi::libinput_device_group_unref, ffi::libinput_device_group_get_user_data, ffi::libinput_device_group_set_user_data);

ffi_ref_struct!(struct Device, ffi::libinput_device, ffi::libinput_device_ref, ffi::libinput_device_unref, ffi::libinput_device_get_user_data, ffi::libinput_device_set_user_data);

impl Device
{
    pub fn context(&self) -> Libinput
    {
        unsafe {
            Libinput::from_raw(ffi::libinput_device_get_context(self.as_raw_mut()))
        }
    }

    pub fn device_group(&self) -> DeviceGroup
    {
        unsafe {
            DeviceGroup::from_raw(ffi::libinput_device_get_device_group(self.as_raw_mut()))
        }
    }

    pub fn sysname(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_sysname(self.as_raw_mut()) ).to_str().expect("Device sysname is no valid utf8")
        }
    }

    pub fn name(&self) -> &str
    {
        unsafe {
            CStr::from_ptr(ffi::libinput_device_get_name(self.as_raw_mut()) ).to_str().expect("Device name is no valid utf8")
        }
    }

    pub fn output_name(&self) -> Option<&str>
    {
        unsafe {
            let ptr = ffi::libinput_device_get_output_name(self.as_raw_mut());
            if !ptr.is_null() {
                Some(CStr::from_ptr(ptr).to_str().expect("Device output_name is no valid utf8"))
            } else {
                None
            }
        }
    }

    ffi_func!(pub fn id_product, ffi::libinput_device_get_id_product, u32);
    ffi_func!(pub fn id_vendor, ffi::libinput_device_get_id_product, u32);

    pub fn seat(&self) -> Seat
    {
        unsafe {
            Seat::from_raw(ffi::libinput_device_get_seat(self.as_raw_mut()))
        }
    }

    pub fn set_seat_logical_name(&mut self, name: &str) -> Result<(), ()>
    {
        let name = CString::new(name).expect("New logical_seat name contained a null-byte");
        unsafe {
            if ffi::libinput_device_set_seat_logical_name(self.as_raw_mut(), name.as_ptr()) == 0 {
                Ok(())
            } else {
                Err(())
            }
        }
    }

    ffi_func!(pub fn udev_device, ffi::libinput_device_get_udev_device, *mut libc::c_void);

    pub fn led_update(&mut self, leds: &[Led]) {
        let mut bitmask = 0u32;
        for led in leds {
            match *led {
                Led::NumLock => bitmask |= ffi::libinput_led_LIBINPUT_LED_NUM_LOCK,
                Led::CapsLock => bitmask |= ffi::libinput_led_LIBINPUT_LED_CAPS_LOCK,
                Led::ScrollLock => bitmask |= ffi::libinput_led_LIBINPUT_LED_SCROLL_LOCK,
            }
        }
        unsafe {
            ffi::libinput_device_led_update(self.as_raw_mut(), bitmask)
        }
    }

    pub fn has_capability(&self, cap: DeviceCapability) -> bool
    {
        unsafe { ffi::libinput_device_has_capability(self.as_raw_mut(), match cap {
            DeviceCapability::Keyboard => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_KEYBOARD,
            DeviceCapability::Pointer => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_POINTER,
            DeviceCapability::Touch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TOUCH,
            DeviceCapability::TabletTool => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_TOOL,
            DeviceCapability::TabletPad => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_PAD,
            DeviceCapability::Gesture => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_GESTURE,
            DeviceCapability::Switch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_SWITCH,
        }) != 0 }
    }

    pub fn get_size(&self) -> Option<(f64, f64)>
    {
        let mut width = 0.0;
        let mut height = 0.0;

        match unsafe { ffi::libinput_device_get_size(self.as_raw_mut(), &mut width as *mut _, &mut height as *mut _) } {
            0 => Some((width, height)),
            _ => None,
        }
    }

    pub fn pointer_has_button(&self, button: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_pointer_has_button(self.as_raw_mut(), button) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    pub fn keyboard_has_key(&self, key: u32) -> Result<bool, ()>
    {
        match unsafe { ffi::libinput_device_keyboard_has_key(self.as_raw_mut(), key) } {
            1 => Ok(true),
            0 => Ok(false),
            -1 => Err(()),
            _ => unreachable!(),
        }
    }

    ffi_func!(pub fn tablet_pad_number_of_buttons, ffi::libinput_device_tablet_pad_get_num_buttons, i32);
    ffi_func!(pub fn tablet_pad_number_of_rings, ffi::libinput_device_tablet_pad_get_num_rings, i32);
    ffi_func!(pub fn tablet_pad_number_of_strips, ffi::libinput_device_tablet_pad_get_num_strips, i32);
    ffi_func!(pub fn tablet_pad_number_of_mode_groups, ffi::libinput_device_tablet_pad_get_num_mode_groups, i32);

    pub fn tablet_pad_get_mode_group(&self, index: u32) -> Option<TabletPadModeGroup> {
        let ptr = unsafe { ffi::libinput_device_tablet_pad_get_mode_group(self.as_raw_mut(), index) };
        if ptr.is_null() {
            None
        } else {
            Some(unsafe { TabletPadModeGroup::from_raw(ptr) })
        }
    }

    pub fn config_accel_default_profile(&self) -> Option<AccelProfile>
    {
        match unsafe { ffi::libinput_device_config_accel_get_default_profile(self.as_raw_mut()) } {
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_NONE => None,
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT => Some(AccelProfile::Flat),
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE => Some(AccelProfile::Adaptive),
        }
    }

    pub fn config_accel_profile(&self) -> Option<AccelProfile>
    {
        match unsafe { ffi::libinput_device_config_accel_get_profile(self.as_raw_mut()) } {
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_NONE => None,
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT => Some(AccelProfile::Flat),
            ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE => Some(AccelProfile::Adaptive),
        }
    }

    pub fn config_accel_profiles(&self) -> Vec<AccelProfile>
    {
        let mut profiles = Vec::new();
        let bitmask = unsafe { ffi::libinput_device_config_accel_get_profiles(self.as_raw_mut()) };
        if bitmask & ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT as u32 == bitmask {
            profiles.push(AccelProfile::Flat);
        }
        if bitmask & ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE as u32 == bitmask {
            profiles.push(AccelProfile::Adaptive);
        }
        profiles
    }

    pub fn config_accel_set_profile(&mut self, profile: AccelProfile) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_accel_set_profile(self.as_raw_mut(), match profile {
            AccelProfile::Flat => ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_FLAT,
            AccelProfile::Adaptive => ffi::libinput_config_accel_profile::LIBINPUT_CONFIG_ACCEL_PROFILE_ADAPTIVE,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_accel_default_speed, ffi::libinput_device_config_accel_get_default_speed, f64);
    ffi_func!(pub fn config_accel_speed, ffi::libinput_device_config_accel_get_speed, f64);

    pub fn config_accel_set_speed(&mut self, speed: f64) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_accel_set_speed(self.as_raw_mut(), speed) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_accel_is_available, ffi::libinput_device_config_accel_is_available, bool);

    pub fn config_calibration_default_matrix(&self) -> Option<[f32; 6]> {
        let mut matrix = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        if unsafe { ffi::libinput_device_config_calibration_get_default_matrix(self.as_raw_mut(), matrix.as_mut_ptr()) != 0 } {
            Some(matrix)
        } else {
            None
        }
    }

    pub fn config_calibration_matrix(&self) -> Option<[f32; 6]> {
        let mut matrix = [0.0, 0.0, 0.0, 0.0, 0.0, 0.0];
        if unsafe { ffi::libinput_device_config_calibration_get_matrix(self.as_raw_mut(), matrix.as_mut_ptr()) != 0 } {
            Some(matrix)
        } else {
            None
        }
    }

    ffi_func!(pub fn config_calibration_has_matrix, ffi::libinput_device_config_calibration_has_matrix, bool);

    pub fn config_calibration_set_matrix(&mut self, matrix: [f32; 6]) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_calibration_set_matrix(self.as_raw_mut(), matrix.as_ptr()) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_click_default_method(&self) -> Option<ClickMethod>
    {
        match unsafe { ffi::libinput_device_config_click_get_default_method(self.as_raw_mut()) } {
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_NONE => None,
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS => Some(ClickMethod::ButtonAreas),
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER => Some(ClickMethod::Clickfinger),
        }
    }

    pub fn config_click_method(&self) -> Option<ClickMethod>
    {
        match unsafe { ffi::libinput_device_config_click_get_method(self.as_raw_mut()) } {
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_NONE => None,
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS => Some(ClickMethod::ButtonAreas),
            ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER => Some(ClickMethod::Clickfinger),
        }
    }

    pub fn config_click_methods(&self) -> Vec<ClickMethod>
    {
        let mut methods = Vec::new();
        let bitmask = unsafe { ffi::libinput_device_config_click_get_methods(self.as_raw_mut()) };
        if bitmask & ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER as u32 == bitmask {
            methods.push(ClickMethod::Clickfinger);
        }
        if bitmask & ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS as u32 == bitmask {
            methods.push(ClickMethod::ButtonAreas);
        }
        methods
    }

    pub fn config_click_set_method(&mut self, method: ClickMethod) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_click_set_method(self.as_raw_mut(), match method {
            ClickMethod::ButtonAreas => ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_BUTTON_AREAS,
            ClickMethod::Clickfinger => ffi::libinput_config_click_method::LIBINPUT_CONFIG_CLICK_METHOD_CLICKFINGER,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_dwt_default_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_dwt_get_default_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_ENABLED => true,
            ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_DISABLED => false,
        }
    }

    pub fn config_dwt_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_dwt_get_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_ENABLED => true,
            ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_DISABLED => false,
        }
    }

    ffi_func!(pub fn config_dwt_is_available, ffi::libinput_device_config_dwt_is_available, bool);

    pub fn config_dwt_set_enabled(&self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_dwt_set_enabled(self.as_raw_mut(), match enabled {
            true => ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_ENABLED,
            false => ffi::libinput_config_dwt_state::LIBINPUT_CONFIG_DWT_DISABLED,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_left_handed, ffi::libinput_device_config_left_handed_get, bool);
    ffi_func!(pub fn config_left_handed_default, ffi::libinput_device_config_left_handed_get_default, bool);
    ffi_func!(pub fn config_left_handed_is_available, ffi::libinput_device_config_left_handed_is_available, bool);


    pub fn config_left_handed_set(&self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_left_handed_set(self.as_raw_mut(), enabled as i32) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_middle_emulation_default_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_middle_emulation_get_default_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_ENABLED => true,
            ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_DISABLED => false,
        }
    }

    pub fn config_middle_emulation_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_middle_emulation_get_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_ENABLED => true,
            ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_DISABLED => false,
        }
    }

    ffi_func!(pub fn config_middle_emulation_is_available, ffi::libinput_device_config_middle_emulation_is_available, bool);

    pub fn config_middle_emulation_set_enabled(&self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_middle_emulation_set_enabled(self.as_raw_mut(), match enabled {
            true => ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_ENABLED,
            false => ffi::libinput_config_middle_emulation_state::LIBINPUT_CONFIG_MIDDLE_EMULATION_DISABLED,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_rotation_angle, ffi::libinput_device_config_rotation_get_angle, u32);
    ffi_func!(pub fn config_rotation_default_angle, ffi::libinput_device_config_rotation_get_default_angle, u32);
    ffi_func!(pub fn config_rotation_is_available, ffi::libinput_device_config_rotation_is_available, bool);

    pub fn config_rotation_set_angle(&self, angle: u32) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_rotation_set_angle(self.as_raw_mut(), angle) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_scroll_button, ffi::libinput_device_config_scroll_get_button, u32);
    ffi_func!(pub fn config_scroll_default_button, ffi::libinput_device_config_scroll_get_default_button, u32);

    pub fn config_scroll_default_method(&self) -> ScrollMethod
    {
        match unsafe { ffi::libinput_device_config_scroll_get_default_method(self.as_raw_mut()) } {
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL => ScrollMethod::NoScroll,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_2FG => ScrollMethod::TwoFinger,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_EDGE => ScrollMethod::Edge,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN => ScrollMethod::OnButtonDown,
        }
    }

    pub fn config_scroll_method(&self) -> ScrollMethod
    {
        match unsafe { ffi::libinput_device_config_scroll_get_method(self.as_raw_mut()) } {
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL => ScrollMethod::NoScroll,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_2FG => ScrollMethod::TwoFinger,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_EDGE => ScrollMethod::Edge,
            ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN => ScrollMethod::OnButtonDown,
        }
    }

    pub fn config_scroll_methods(&self) -> Vec<ScrollMethod>
    {
        let mut methods = Vec::new();
        let bitmask = unsafe { ffi::libinput_device_config_scroll_get_methods(self.as_raw_mut()) };
        if bitmask & ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL as u32 == bitmask {
            methods.push(ScrollMethod::NoScroll);
        }
        if bitmask & ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_2FG as u32 == bitmask {
            methods.push(ScrollMethod::TwoFinger);
        }
        if bitmask & ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_EDGE as u32 == bitmask {
            methods.push(ScrollMethod::Edge);
        }
        if bitmask & ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN as u32 == bitmask {
            methods.push(ScrollMethod::OnButtonDown);
        }
        methods
    }

    pub fn config_scroll_set_method(&mut self, method: ScrollMethod) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_scroll_set_method(self.as_raw_mut(), match method {
            ScrollMethod::NoScroll => ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_NO_SCROLL,
            ScrollMethod::TwoFinger => ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_2FG,
            ScrollMethod::Edge => ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_EDGE,
            ScrollMethod::OnButtonDown => ffi::libinput_config_scroll_method::LIBINPUT_CONFIG_SCROLL_ON_BUTTON_DOWN,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    ffi_func!(pub fn config_scroll_default_natural_scroll_enabled, ffi::libinput_device_config_scroll_get_default_natural_scroll_enabled, bool);
    ffi_func!(pub fn config_scroll_natural_scroll_enabled, ffi::libinput_device_config_scroll_get_natural_scroll_enabled, bool);
    ffi_func!(pub fn config_scroll_has_natural_scroll, ffi::libinput_device_config_scroll_has_natural_scroll, bool);

    pub fn config_scroll_set_natural_scroll_enabled(&mut self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_scroll_set_natural_scroll_enabled(self.as_raw_mut(), enabled as i32) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_scroll_set_button(&mut self, button: u32) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_scroll_set_button(self.as_raw_mut(), button) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_send_events_mode(&self) -> Vec<SendEventsMode> {
        let mut methods = Vec::new();
        let bitmask = unsafe { ffi::libinput_device_config_send_events_get_mode(self.as_raw_mut()) };
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_ENABLED as u32 == bitmask {
            methods.push(SendEventsMode::Enabled);
        }
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED as u32 == bitmask {
            methods.push(SendEventsMode::Disabled);
        }
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED_ON_EXTERNAL_MOUSE as u32 == bitmask {
            methods.push(SendEventsMode::DisabledOnExternalMouse);
        }
        methods
    }

    pub fn config_send_events_modes(&self) -> Vec<SendEventsMode> {
        let mut methods = Vec::new();
        let bitmask = unsafe { ffi::libinput_device_config_send_events_get_modes(self.as_raw_mut()) };
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_ENABLED as u32 == bitmask {
            methods.push(SendEventsMode::Enabled);
        }
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED as u32 == bitmask {
            methods.push(SendEventsMode::Disabled);
        }
        if bitmask & ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED_ON_EXTERNAL_MOUSE as u32 == bitmask {
            methods.push(SendEventsMode::DisabledOnExternalMouse);
        }
        methods
    }

    pub fn config_send_events_set_mode(&self, mode: &[SendEventsMode]) -> DeviceConfigResult {
        let mut bitmask = 0u32;
        for flag in mode {
            match *flag {
                SendEventsMode::Enabled => bitmask |= ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_ENABLED as u32,
                SendEventsMode::Disabled => bitmask |= ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED as u32,                SendEventsMode::DisabledOnExternalMouse => bitmask |= ffi::libinput_config_send_events_mode_LIBINPUT_CONFIG_SEND_EVENTS_DISABLED_ON_EXTERNAL_MOUSE as u32,
            }
        }
        match unsafe { ffi::libinput_device_config_send_events_set_mode(self.as_raw_mut(), bitmask) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_tap_button_map(&self) -> Option<TapButtonMap> {
        if self.config_tap_finger_count() == 0 {
            None
        } else {
            match unsafe { ffi::libinput_device_config_tap_get_button_map(self.as_raw_mut()) } {
                ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LRM => Some(TapButtonMap::LeftRightMiddle),
                ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LMR => Some(TapButtonMap::LeftMiddleRight),
            }
        }
    }

    pub fn config_tap_default_button_map(&self) -> Option<TapButtonMap> {
        if self.config_tap_finger_count() == 0 {
            None
        } else {
            match unsafe { ffi::libinput_device_config_tap_get_default_button_map(self.as_raw_mut()) } {
                ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LRM => Some(TapButtonMap::LeftRightMiddle),
                ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LMR => Some(TapButtonMap::LeftMiddleRight),
            }
        }
    }

    pub fn config_tap_default_drag_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_default_drag_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_ENABLED => true,
            ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_DISABLED => false,
        }
    }

    pub fn config_tap_default_drag_lock_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_default_drag_lock_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_ENABLED => true,
            ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_DISABLED => false,
        }
    }

    pub fn config_tap_default_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_default_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_ENABLED => true,
            ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_DISABLED => false,
        }
    }

    pub fn config_tap_drag_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_drag_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_ENABLED => true,
            ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_DISABLED => false,
        }
    }

    pub fn config_tap_drag_lock_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_drag_lock_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_ENABLED => true,
            ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_DISABLED => false,
        }
    }

    pub fn config_tap_enabled(&self) -> bool {
        match unsafe { ffi::libinput_device_config_tap_get_enabled(self.as_raw_mut()) } {
            ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_ENABLED => true,
            ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_DISABLED => false,
        }
    }

    ffi_func!(pub fn config_tap_finger_count, ffi::libinput_device_config_tap_get_finger_count, u32);

    pub fn config_tap_set_button_map(&mut self, map: TapButtonMap) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_tap_set_button_map(self.as_raw_mut(), match map {
            TapButtonMap::LeftRightMiddle => ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LRM,
            TapButtonMap::LeftMiddleRight => ffi::libinput_config_tap_button_map::LIBINPUT_CONFIG_TAP_MAP_LMR,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_tap_set_drag_enabled(&mut self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_tap_set_drag_enabled(self.as_raw_mut(), match enabled {
            true => ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_ENABLED,
            false => ffi::libinput_config_drag_state::LIBINPUT_CONFIG_DRAG_DISABLED,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_tap_set_drag_lock_enabled(&mut self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_tap_set_drag_lock_enabled(self.as_raw_mut(), match enabled {
            true => ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_ENABLED,
            false => ffi::libinput_config_drag_lock_state::LIBINPUT_CONFIG_DRAG_LOCK_DISABLED,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }

    pub fn config_tap_set_enabled(&mut self, enabled: bool) -> DeviceConfigResult {
        match unsafe { ffi::libinput_device_config_tap_set_enabled(self.as_raw_mut(), match enabled {
            true => ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_ENABLED,
            false => ffi::libinput_config_tap_state::LIBINPUT_CONFIG_TAP_DISABLED,
        }) } {
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_SUCCESS => Ok(()),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_UNSUPPORTED => Err(DeviceConfigError::Unsupported),
            ffi::libinput_config_status::LIBINPUT_CONFIG_STATUS_INVALID => Err(DeviceConfigError::Invalid),
        }
    }
}
