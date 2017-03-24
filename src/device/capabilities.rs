use ::ffi;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum LibinputDeviceCapability
{
    Keyboard,
    Pointer,
    Touch,
    TabletTool,
    TabletPad,
    Gesture,
    Switch,
}

impl From<ffi::libinput_device_capability> for LibinputDeviceCapability
{
    fn from(ffi: ffi::libinput_device_capability) -> Self {
        match ffi {
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_KEYBOARD => LibinputDeviceCapability::Keyboard,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_POINTER => LibinputDeviceCapability::Pointer,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TOUCH => LibinputDeviceCapability::Touch,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_TOOL => LibinputDeviceCapability::TabletTool,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_PAD => LibinputDeviceCapability::TabletPad,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_GESTURE => LibinputDeviceCapability::Gesture,
            ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_SWITCH => LibinputDeviceCapability::Switch,
        }
    }
}

impl From<LibinputDeviceCapability> for ffi::libinput_device_capability
{
    fn from(cap: LibinputDeviceCapability) -> Self {
        match cap {
            LibinputDeviceCapability::Keyboard => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_KEYBOARD,
            LibinputDeviceCapability::Pointer => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_POINTER,
            LibinputDeviceCapability::Touch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TOUCH,
            LibinputDeviceCapability::TabletTool => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_TOOL,
            LibinputDeviceCapability::TabletPad => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_TABLET_PAD,
            LibinputDeviceCapability::Gesture => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_GESTURE,
            LibinputDeviceCapability::Switch => ffi::libinput_device_capability::LIBINPUT_DEVICE_CAP_SWITCH,
        }
    }
}
