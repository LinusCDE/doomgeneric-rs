/// Information about a key getting pressed or released
#[derive(Debug, Clone, Copy)]
pub struct KeyData {
    pub pressed: bool,
    pub key: u8,
}

mod key_bindings {
    use std::os::raw;

    extern "C" {
        pub static key_right: raw::c_int;
        pub static key_left: raw::c_int;
        pub static key_up: raw::c_int;
        pub static key_down: raw::c_int;
        pub static key_strafeleft: raw::c_int;
        pub static key_straferight: raw::c_int;
        pub static key_fire: raw::c_int;
        pub static key_use: raw::c_int;
        pub static key_strafe: raw::c_int;
        pub static key_speed: raw::c_int;
    }
}

/// Common keys used in doom
/// Keys are based on keyboard keycodes. So extra keys, like letters should be available when provided the correct keycode.
/// ASCII chars for example can just be cast to a u8 to work.
pub mod keys {
    use super::key_bindings;
    use once_cell::sync::Lazy;

    pub static KEY_RIGHT: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_right } as u8);
    pub static KEY_LEFT: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_left } as u8);
    pub static KEY_UP: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_up } as u8);
    pub static KEY_DOWN: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_down } as u8);
    pub static KEY_STRAFELEFT: Lazy<u8> =
        Lazy::new(|| unsafe { key_bindings::key_strafeleft } as u8);
    pub static KEY_STRAFERIGHT: Lazy<u8> =
        Lazy::new(|| unsafe { key_bindings::key_straferight } as u8);
    pub static KEY_FIRE: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_fire } as u8);
    pub static KEY_USE: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_use } as u8);
    /// When pressed, KEY_LEFT and KEY_RIGHT act like KEY_STRAFELEFT and KEY_STRAFERIGHT accordingly
    pub static KEY_STRAFE: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_strafe } as u8);
    pub static KEY_SPEED: Lazy<u8> = Lazy::new(|| unsafe { key_bindings::key_speed } as u8);

    pub static KEY_ESCAPE: u8 = 27;
    pub static KEY_ENTER: u8 = '\r' as u8;

    pub fn from_char(ascii_char: char) -> Option<u8> {
        if ascii_char.is_ascii() {
            Some(ascii_char as u8)
        } else {
            None
        }
    }
}
