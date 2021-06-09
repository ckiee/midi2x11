use std::ffi::CString;
use std::mem;
use std::os::raw::*;
use std::ptr;

use anyhow::Result;
use x11_dl::{xlib, xtest::Xf86vmode};
use xlib::Xlib;

pub struct KeyPresser {
    display_handle: *mut x11_dl::xlib::Display,
    xlib: Xlib,
    xf86vm: Xf86vmode,
}

impl KeyPresser {
    pub fn new() -> Self {
        unsafe {
            let xlib = Xlib::open().unwrap();
            let xf86vm = Xf86vmode::open().unwrap();
            let display = (xlib.XOpenDisplay)(ptr::null());

            if display.is_null() {
                panic!("XOpenDisplay failed");
            }

            KeyPresser {
                display_handle: display,
                xlib,
                xf86vm,
            }
        }
    }
    // 0x74 or 0x79
    pub fn send_key_event(&self, down: bool, keycode: u32) {
        unsafe {
            if keycode != 0 {
                (self.xf86vm.XTestFakeKeyEvent)(self.display_handle, keycode, down.into(), 0);
                (self.xlib.XFlush)(self.display_handle);
            }
        }
    }
    pub fn get_keycode(&self, name: &str) -> Result<u8> {
        let cstring = CString::new(name)?;
        unsafe {
            let keysym = (self.xlib.XStringToKeysym)(cstring.as_c_str().as_ptr());
            Ok((self.xlib.XKeysymToKeycode)(self.display_handle, keysym))
        }
    }
}

impl Drop for KeyPresser {
    fn drop(&mut self) {
        unsafe {
            (self.xlib.XCloseDisplay)(self.display_handle);
        }
    }
}
