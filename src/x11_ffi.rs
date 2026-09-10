use std::ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort};

#[repr(C)]
pub struct _XkbStateRec {
    pub group: c_uchar,
    pub base_group: c_ushort,
    pub latched_group: c_ushort,
    pub locked_group: c_uchar,
    pub mods: c_uchar,
    pub base_mods: c_uchar,
    pub latched_mods: c_uchar,
    pub locked_mods: c_uchar,
    pub compat_state: c_uchar,
    pub grab_mods: c_uchar,
    pub compat_grab_mods: c_uchar,
    pub lookup_mods: c_uchar,
    pub compat_lookup_mods: c_uchar,
    pub ptr_buttons: c_ushort,
}

#[repr(C)]
pub struct _XkbRF_VarDefs {
    pub model: *mut c_char,
    pub layout: *mut c_char,
    pub variant: *mut c_char,
    pub options: *mut c_char,
    pub sz_extra: c_ushort,
    pub num_extra: c_ushort,
    pub extra_names: *mut c_char,
    pub extra_values: *mut c_char,
}

pub enum _XDisplay {}

pub type Display = _XDisplay;
pub type XkbStatePtr = *mut _XkbStateRec;
#[allow(non_camel_case_types)]
pub type XkbRF_VarDefsPtr = *mut _XkbRF_VarDefs;

#[allow(non_upper_case_globals)]
pub static XkbUseCoreKbd: c_uint = 0x0100;

extern "C" {
    pub fn XOpenDisplay(_1: *const c_char) -> *mut Display;
    pub fn XRootWindow(_2: *mut Display, _1: c_int) -> c_ulong;
    pub fn XDefaultScreen(_1: *mut Display) -> c_int;
    pub fn XStoreName(_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
    pub fn XCloseDisplay(_1: *mut Display) -> c_int;
    pub fn XkbGetState(_3: *mut Display, _2: c_uint, _1: XkbStatePtr) -> c_int;
    pub fn XkbRF_GetNamesProp(_3: *mut Display, _2: *const c_char, _1: XkbRF_VarDefsPtr) -> bool;
}
