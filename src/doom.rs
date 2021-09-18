use lazy_static::lazy_static;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::os::raw;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::{Duration, Instant};

const DOOMGENERIC_RESX: usize = 640;
const DOOMGENERIC_RESY: usize = 480;

#[derive(Debug, Clone, Copy)]
pub struct KeyData {
    pub pressed: bool,
    pub key: u8,
}

pub trait Doom: Send + Sync {
    fn draw_frame(&mut self, screen_buffer: &[u32], xres: usize, yres: usize);
    fn get_key(&mut self) -> Option<KeyData>;
    fn set_window_title(&mut self, title: &str);
}

extern "C" {
    fn D_DoomMain(); // doomgeneric.h
    fn M_FindResponseFile(); // used in main of i_main.c
}

#[no_mangle]
static mut DG_ScreenBuffer: *const u32 = std::ptr::null();
//static DG_ScreenBuffer: &[u32] = &[0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY];

lazy_static! {
    static ref START_TIME: Instant = Instant::now();
    static ref SCREEN_BUFFER: Mutex<Option<Box<[u32]>>> = Mutex::new(None);
    static ref DOOM_HANDLER: Mutex<Option<Box<dyn Doom>>> = Mutex::new(None);
}

#[no_mangle]
extern "C" fn DG_Init() {
    println!("DG_Init()");
    println!("Passed millis: {}", DG_GetTicksMs());

    *SCREEN_BUFFER.lock().unwrap() = Some(Box::new([0u32; DOOMGENERIC_RESX * DOOMGENERIC_RESY]));
    unsafe {
        // Setting DG_ScreenBuffer to where the new buffer is
        DG_ScreenBuffer = SCREEN_BUFFER.lock().unwrap().as_ref().unwrap().as_ptr();
    }
}

#[no_mangle]
extern "C" fn DG_GetKey(pressed: *mut raw::c_int, key: *mut raw::c_uchar) -> raw::c_int {
    let has_key = if let Some(doom_box) = DOOM_HANDLER.lock().unwrap().as_mut() {
        if let Some(keydata) = doom_box.get_key() {
            unsafe {
                // Not tested yet!
                *pressed = if keydata.pressed { 1 } else { 0 };
                *key = keydata.key;
            }
            true
        } else {
            false
        }
    } else {
        false
    };

    println!("DG_GetKey(<pressedPtr>, <doomKeyPtr>) -> {}", has_key);
    raw::c_int::from(has_key)
}

#[no_mangle]
extern "C" fn DG_GetTicksMs() -> u32 {
    let tick_ms = u32::try_from(START_TIME.elapsed().as_millis())
        .expect("Can't fit passed milliseconds into u32!");
    println!("DG_GetTicksMs() -> {:?}", tick_ms);
    tick_ms
}

#[no_mangle]
extern "C" fn DG_SleepMs(ms: u32) {
    println!("DG_SleepMs({:?})", ms);
    sleep(Duration::from_millis(ms as u64));
}

#[no_mangle]
extern "C" fn DG_DrawFrame() {
    println!("DG_DrawFrame()");
    if let Some(doom_box) = DOOM_HANDLER.lock().unwrap().as_mut() {
        if let Some(screen_buffer) = SCREEN_BUFFER.lock().unwrap().as_mut() {
            doom_box.draw_frame(screen_buffer, DOOMGENERIC_RESX, DOOMGENERIC_RESY);
        }
    }
}

#[no_mangle]
extern "C" fn DG_SetWindowTitle(title: *const raw::c_char) {
    let title = unsafe { CStr::from_ptr(title) }
        .to_str()
        .expect("Can't convert title c string to rust string");
    println!("DG_SetWindowTitle({:?})", title);
    if let Some(doom_box) = DOOM_HANDLER.lock().unwrap().as_mut() {
        doom_box.set_window_title(title);
    }
}

pub fn init(doom_impl: impl Doom + 'static) {
    unsafe {
        *DOOM_HANDLER.lock().unwrap() = Some(Box::new(doom_impl));

        M_FindResponseFile();
        DG_Init();
        D_DoomMain();
    }
}