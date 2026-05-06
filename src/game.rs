use crate::input::KeyData;
use once_cell::sync::Lazy;
use std::convert::TryFrom;
use std::ffi::{CStr, CString};
use std::os::raw;
use std::ptr::addr_of_mut;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// Resolution defined as overridable macros in the underlying C library.
/// Override at build time via `DOOMGENERIC_RESX` / `DOOMGENERIC_RESY` env vars.
pub const DOOMGENERIC_RESX: usize = match usize::from_str_radix(env!("DOOMGENERIC_RESX"), 10) {
    Ok(v) => v,
    Err(_) => 640,
};
pub const DOOMGENERIC_RESY: usize = match usize::from_str_radix(env!("DOOMGENERIC_RESY"), 10) {
    Ok(v) => v,
    Err(_) => 400,
};

pub trait DoomGeneric {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize);
    fn get_key(&mut self) -> Option<KeyData>;
    fn set_window_title(&mut self, title: &str);
}

extern "C" {
    fn doomgeneric_Create(argc: raw::c_int, argv: *mut *mut raw::c_char);
    fn doomgeneric_Tick();
    static mut DG_ScreenBuffer: *mut u32;
}

static mut DOOM_HANDLER: Option<Box<dyn DoomGeneric>> = None;
static START_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());

#[no_mangle]
extern "C" fn DG_Init() {}

#[no_mangle]
extern "C" fn DG_GetKey(pressed: *mut raw::c_int, key: *mut raw::c_uchar) -> raw::c_int {
    if let Some(doom_box) = unsafe { (*addr_of_mut!(DOOM_HANDLER)).as_mut() } {
        if let Some(keydata) = doom_box.get_key() {
            unsafe {
                // Not tested yet!
                *pressed = if keydata.pressed { 1 } else { 0 };
                *key = keydata.key;
            }
            1
        } else {
            0
        }
    } else {
        0
    }
}

#[no_mangle]
extern "C" fn DG_GetTicksMs() -> u32 {
    u32::try_from(START_TIME.elapsed().as_millis())
        .expect("Can't fit passed milliseconds into u32!")
}

#[no_mangle]
extern "C" fn DG_SleepMs(ms: u32) {
    sleep(Duration::from_millis(ms as u64));
}

#[no_mangle]
extern "C" fn DG_DrawFrame() {
    if let Some(doom_box) = unsafe { (*addr_of_mut!(DOOM_HANDLER)).as_mut() } {
        let buf = unsafe { DG_ScreenBuffer };
        if !buf.is_null() {
            let slice = unsafe {
                std::slice::from_raw_parts(buf, DOOMGENERIC_RESX * DOOMGENERIC_RESY)
            };
            doom_box.draw_frame(slice, DOOMGENERIC_RESX, DOOMGENERIC_RESY);
        }
    }
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(title: *const raw::c_char) {
    let title = unsafe { CStr::from_ptr(title) }
        .to_str()
        .expect("Can't convert title c string to rust string");
    if let Some(doom_box) = unsafe { (*addr_of_mut!(DOOM_HANDLER)).as_mut() } {
        doom_box.set_window_title(title);
    }
}

pub fn init<S: AsRef<str>>(args: &[S], doom_impl: impl DoomGeneric + 'static) {
    let cstrings: Vec<CString> = args
        .iter()
        .map(|s| CString::new(s.as_ref()).expect("argv contains nul byte"))
        .collect();
    let mut argv_ptrs: Vec<*mut raw::c_char> = cstrings
        .iter()
        .map(|cs| cs.as_ptr() as *mut raw::c_char)
        .collect();
    argv_ptrs.push(std::ptr::null_mut());
    let argc = cstrings.len() as raw::c_int;
    let argv = argv_ptrs.as_mut_ptr();
    std::mem::forget(cstrings);
    std::mem::forget(argv_ptrs);

    unsafe {
        *addr_of_mut!(DOOM_HANDLER) = Some(Box::new(doom_impl));
        doomgeneric_Create(argc, argv);
    }
}

pub fn tick() {
    unsafe {
        doomgeneric_Tick();
    }
}
