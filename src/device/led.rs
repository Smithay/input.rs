use ::ffi;

bitflags! {
    pub flags LibinputLed: u32 {
        const NUM_LOCK = ffi::libinput_led_LIBINPUT_LED_NUM_LOCK,
        const CAPS_LOCK = ffi::libinput_led_LIBINPUT_LED_CAPS_LOCK,
        const SCROLL_LOCK = ffi::libinput_led_LIBINPUT_LED_SCROLL_LOCK,
    }
}
