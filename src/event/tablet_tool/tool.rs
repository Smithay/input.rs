use std::marker::PhantomData;
use std::mem;
use std::ptr;

use libc;

use ::{ffi, FromRaw, AsRaw, Userdata};

#[derive(Debug, Clone, Copy)]
pub enum TabletToolType {
    Pen,
    Eraser,
    Brush,
    Pencil,
    Airbrush,
    Mouse,
    Lens,
}

pub struct TabletTool<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> {
    tool: *mut ffi::libinput_tablet_tool,
    _context_userdata_type: PhantomData<C>,
    _device_userdata_type: PhantomData<D>,
    _device_group_userdata_type: PhantomData<G>,
    _seat_userdata_type: PhantomData<S>,
    _tablet_tool_userdata_type: PhantomData<T>,
    _tablet_pad_mode_group_userdata_type: PhantomData<M>,
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> TabletTool<C, D, G, S, T, M> {
    pub fn serial(&self) -> u64 {
        unsafe { ffi::libinput_tablet_tool_get_serial(self.as_raw() as *mut _) }
    }

    pub fn tool_id(&self) -> u64 {
        unsafe { ffi::libinput_tablet_tool_get_tool_id(self.as_raw() as *mut _) }
    }

    pub fn tool_type(&self) -> TabletToolType {
        match unsafe { ffi::libinput_tablet_tool_get_type(self.as_raw() as *mut _) } {
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
        unsafe { ffi::libinput_tablet_tool_has_button(self.as_raw() as *mut _, button) != 0 }
    }

    pub fn has_distance(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_distance(self.as_raw() as *mut _) != 0 }
    }

    pub fn has_pressure(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_pressure(self.as_raw() as *mut _) != 0 }
    }

    pub fn has_rotation(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_rotation(self.as_raw() as *mut _) != 0 }
    }

    pub fn has_slider(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_slider(self.as_raw() as *mut _) != 0 }
    }

    pub fn has_tilt(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_tilt(self.as_raw() as *mut _) != 0 }
    }

    pub fn has_wheel(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_has_wheel(self.as_raw() as *mut _) != 0 }
    }

    pub fn is_unique(&self) -> bool {
        unsafe { ffi::libinput_tablet_tool_is_unique(self.as_raw() as *mut _) != 0 }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  FromRaw<ffi::libinput_tablet_tool> for TabletTool<C, D, G, S, T, M>
{
    unsafe fn from_raw(raw: *mut ffi::libinput_tablet_tool) -> TabletTool<C, D, G, S, T, M>
    {
        TabletTool {
            tool: ffi::libinput_tablet_tool_ref(raw),
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  AsRaw<ffi::libinput_tablet_tool> for TabletTool<C, D, G, S, T, M>
{
    unsafe fn as_raw(&self) -> *const ffi::libinput_tablet_tool {
        self.tool as *const _
    }

    unsafe fn as_raw_mut(&mut self) -> *mut ffi::libinput_tablet_tool {
        self.tool as *mut _
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Userdata<T> for TabletTool<C, D, G, S, T, M>
{
    fn userdata(&self) -> Option<&T> {
        unsafe {
            (ffi::libinput_tablet_tool_get_user_data(self.tool) as *const T).as_ref()
        }
    }

    fn userdata_mut(&mut self) -> Option<&mut T> {
        unsafe {
            (ffi::libinput_tablet_tool_get_user_data(self.tool) as *mut T).as_mut()
        }
    }

    fn set_userdata(&mut self, userdata: Option<T>) -> Option<T> {
        let old = unsafe {
            let ptr = ffi::libinput_tablet_tool_get_user_data(self.tool);
            if !ptr.is_null() {
                Some(Box::from_raw(ptr as *mut T))
            } else {
                None
            }
        };
        let mut boxed = Box::new(userdata);
        unsafe {
            ffi::libinput_tablet_tool_set_user_data(self.tool, match (*boxed).as_mut() {
                Some(value) => value as *mut T as *mut libc::c_void,
                None => ptr::null_mut(),
            });
        }
        mem::forget(boxed);
        old.map(|x| *x)
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static>  Clone for TabletTool<C, D, G, S, T, M>
{
    fn clone(&self) -> TabletTool<C, D, G, S, T, M>
    {
        TabletTool {
            tool: unsafe { ffi::libinput_tablet_tool_ref(self.tool) },
            _context_userdata_type: PhantomData,
            _device_userdata_type: PhantomData,
            _device_group_userdata_type: PhantomData,
            _seat_userdata_type: PhantomData,
            _tablet_tool_userdata_type: PhantomData,
            _tablet_pad_mode_group_userdata_type: PhantomData,
        }
    }
}

impl<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static, M: 'static> Drop for TabletTool<C, D, G, S, T, M>
{
    fn drop(&mut self) {
        unsafe {
            let userdata_ref = ffi::libinput_tablet_tool_get_user_data(self.tool);
            if ffi::libinput_tablet_tool_unref(self.tool).is_null() {
                Box::from_raw(userdata_ref);
            }
        }
    }
}
