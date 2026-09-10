use crate::x11_ffi::*;
use std::{
    ffi::{c_ulong, CStr, CString},
    mem::MaybeUninit,
    ptr,
};

pub struct X11Bar {
    display: *mut Display,
    window: c_ulong,
}

impl X11Bar {
    pub fn new() -> Result<Self, &'static str> {
        let display = unsafe { XOpenDisplay(ptr::null()) };
        if display.is_null() {
            return Err("Cannot open display");
        }
        let window = unsafe { XRootWindow(display, XDefaultScreen(display)) };

        Ok(Self { display, window })
    }

    pub fn xsetroot<T: AsRef<str>>(&self, new_name: T) {
        let name = CString::new(new_name.as_ref()).unwrap();
        unsafe { XStoreName(self.display, self.window, name.as_ptr()) };
    }

    pub fn close_display(&self) {
        unsafe { XCloseDisplay(self.display) };
    }

    pub fn kbd_layout(&self) -> &str {
        let (vd, state) = unsafe {
            let mut state: MaybeUninit<_XkbStateRec> = MaybeUninit::uninit();
            let _ = XkbGetState(self.display, XkbUseCoreKbd, state.as_mut_ptr());

            let mut vd: MaybeUninit<_XkbRF_VarDefs> = MaybeUninit::uninit();
            let _ = XkbRF_GetNamesProp(self.display, ptr::null(), vd.as_mut_ptr());

            (
                CStr::from_ptr(vd.assume_init().layout).to_str().unwrap(),
                state.assume_init().group,
            )
        };

        vd.split(",").collect::<Vec<&str>>()[state as usize]
    }
}
