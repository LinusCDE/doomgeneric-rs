use crate::input::KeyData;
use once_cell::sync::Lazy;
use std::cell::RefCell;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw;
use std::thread::sleep;
use std::time::{Duration, Instant};

/// The resolution is hardcoded in the underlying library as macro definitions.
pub const DOOMGENERIC_RESX: usize = 640;
pub const DOOMGENERIC_RESY: usize = 400;

pub trait DoomGeneric {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize);
    fn get_key(&mut self) -> Option<KeyData>;
    fn set_window_title(&mut self, title: &str);
}

// TODO: Migrate to doomgeneric_Create

extern "C" {
    fn D_DoomMain(); // doomgeneric.h
    fn doomgeneric_Tick(); // doomgeneric.h
    fn M_FindResponseFile(); // used in main of i_main.c
    pub static mut myargc: raw::c_int;
    pub static mut myargv: *mut *mut raw::c_char;
}

#[no_mangle]
static mut DG_ScreenBuffer: *const u32 = std::ptr::null();
//static DG_ScreenBuffer: &[u32] = &[0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY];
static mut SCREEN_BUFFER: RefCell<Option<Box<[u32]>>> = RefCell::new(None);
static mut DOOM_HANDLER: RefCell<Option<Box<dyn DoomGeneric>>> = RefCell::new(None);
static START_TIME: Lazy<Instant> = Lazy::new(|| Instant::now());

#[no_mangle]
extern "C" fn DG_Init() {
    unsafe {
        *SCREEN_BUFFER.get_mut() = Some(Box::new([0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY]));
        // Setting DG_ScreenBuffer to where the new buffer is
        DG_ScreenBuffer = SCREEN_BUFFER.get_mut().as_ref().unwrap().as_ptr();
    }
}

#[no_mangle]
extern "C" fn DG_GetKey(pressed: *mut raw::c_int, key: *mut raw::c_uchar) -> raw::c_int {
    if let Some(doom_box) = unsafe { DOOM_HANDLER.get_mut().as_mut() } {
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
    if let Some(doom_box) = unsafe { DOOM_HANDLER.get_mut() }.as_mut() {
        if let Some(screen_buffer) = unsafe { SCREEN_BUFFER.get_mut() }.as_mut() {
            doom_box.draw_frame(screen_buffer, DOOMGENERIC_RESX, DOOMGENERIC_RESY);
        }
    }
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(title: *const raw::c_char) {
    let title = unsafe { CStr::from_ptr(title) }
        .to_str()
        .expect("Can't convert title c string to rust string");
    if let Some(doom_box) = unsafe { DOOM_HANDLER.get_mut() }.as_mut() {
        doom_box.set_window_title(title);
    }
}

pub fn init(doom_impl: impl DoomGeneric + 'static) {
    unsafe {
        *DOOM_HANDLER.get_mut() = Some(Box::new(doom_impl));

        M_FindResponseFile();
        DG_Init();
        D_DoomMain();
    }
}

pub fn tick() {
    unsafe {
        doomgeneric_Tick();
    }
}
