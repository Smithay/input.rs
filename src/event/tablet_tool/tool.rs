use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TabletToolType {
    Pen,
    Eraser,
    Brush,
    Pencil,
    Airbrush,
    Mouse,
    Lens,
}

ffi_ref_struct!(TabletTool, ffi::libinput_tablet_tool, T, ffi::libinput_tablet_tool_ref, ffi::libinput_tablet_tool_unref, ffi::libinput_tablet_tool_get_user_data, ffi::libinput_tablet_tool_set_user_data);

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletTool<C, D, G, S, T, M> {
    ffi_func!(pub serial, ffi::libinput_tablet_tool_get_serial, u64);
    ffi_func!(pub tool_id, ffi::libinput_tablet_tool_get_tool_id, u64);

    pub fn tool_type(&self) -> TabletToolType {
        match unsafe { ffi::libinput_tablet_tool_get_type(self.as_raw_mut()) } {
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_PEN => TabletToolType::Pen,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_ERASER => TabletToolType::Eraser,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_BRUSH => TabletToolType::Brush,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_PENCIL => TabletToolType::Pencil,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_AIRBRUSH => TabletToolType::Airbrush,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_MOUSE => TabletToolType::Mouse,
            ffi::libinput_tablet_tool_type::LIBINPUT_TABLET_TOOL_TYPE_LENS => TabletToolType::Lens,
        }
    }

    pub fn has_button(&self, button: u32) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_button(self.as_raw_mut(), button) != 0 }
    }

    ffi_func!(pub has_distance, ffi::libinput_tablet_tool_has_distance, bool);
    ffi_func!(pub has_pressure, ffi::libinput_tablet_tool_has_pressure, bool);
    ffi_func!(pub has_rotation, ffi::libinput_tablet_tool_has_rotation, bool);
    ffi_func!(pub has_slider, ffi::libinput_tablet_tool_has_slider, bool);
    ffi_func!(pub has_tilt, ffi::libinput_tablet_tool_has_tilt, bool);
    ffi_func!(pub has_wheel, ffi::libinput_tablet_tool_has_wheel, bool);
    ffi_func!(pub is_unique, ffi::libinput_tablet_tool_is_unique, bool);
}
