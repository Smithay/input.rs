mod device;
mod gesture;
mod key;
mod pointer;
mod switch;
mod tablet_tool;
mod tablet_pad;
mod touch;

pub use self::device::*;
pub use self::gesture::*;
pub use self::key::*;
pub use self::pointer::*;
pub use self::switch::*;
pub use self::tablet_tool::*;
pub use self::tablet_pad::*;
pub use self::touch::*;

use ::{LibinputContext, LibinputDevice};

pub enum LibinputEvent<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    Device(DeviceEvent<C, D, G, S, T>),
    Keyboard(KeyboardEvent<C, D, G, S, T>),
    Pointer(PointerEvent<C, D, G, S, T>),
    Touch(TouchEvent<C, D, G, S, T>),
    Tablet(TabletToolEvent<C, D, G, S, T>),
    TabletPad(TabletPadEvent<C, D, G, S, T>),
    Gesture(GestureEvent<C, D, G, S, T>),
    Switch(SwitchEvent<C, D, G, S, T>),
}

pub trait Event<C: 'static, D: 'static, G: 'static, S: 'static, T: 'static> {
    fn context(&self) -> LibinputContext<C, D, G, S, T>;
    fn device(&self) -> LibinputDevice<C, D, G, S, T>;
}
