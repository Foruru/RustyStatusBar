use argparse::{ArgumentParser, Store, StoreTrue};
use chrono::Local;
use std::{
    ffi::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort, CStr, CString},
    fs::File,
    io::Read,
    mem::MaybeUninit,
    ptr,
    thread::sleep,
    time::Duration,
};

#[repr(C)]
struct _XkbStateRec {
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

enum _XDisplay {}

type Display = _XDisplay;
type XkbStatePtr = *mut _XkbStateRec;
#[allow(non_camel_case_types)]
type XkbRF_VarDefsPtr = *mut _XkbRF_VarDefs;

#[allow(non_upper_case_globals)]
static XkbUseCoreKbd: c_uint = 0x0100;

extern "C" {
    fn XOpenDisplay(_1: *const c_char) -> *mut Display;
    fn XRootWindow(_2: *mut Display, _1: c_int) -> c_ulong;
    fn XDefaultScreen(_1: *mut Display) -> c_int;
    fn XStoreName(_3: *mut Display, _2: c_ulong, _1: *const c_char) -> c_int;
    fn XCloseDisplay(_1: *mut Display) -> c_int;
    fn XkbGetState(_3: *mut Display, _2: c_uint, _1: XkbStatePtr) -> c_int;
    fn XkbRF_GetNamesProp(_3: *mut Display, _2: *const c_char, _1: XkbRF_VarDefsPtr) -> bool;
}

struct X11Bar {
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

    pub fn run(&self, is_looped: bool, refresh_rate: Duration) {
        self.xsetroot(self.statusbar());
        while is_looped {
            sleep(refresh_rate);
            self.xsetroot(self.statusbar());
        }

        self.close_display();
    }

    fn xsetroot<T: AsRef<str>>(&self, new_name: T) {
        let name = CString::new(new_name.as_ref()).unwrap();
        unsafe { XStoreName(self.display, self.window, name.as_ptr()) };
    }

    fn close_display(&self) {
        unsafe { XCloseDisplay(self.display) };
    }

    fn kbd_layout(&self) -> &str {
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

trait StatusBar {
    fn statusbar(&self) -> String;
}

impl StatusBar for X11Bar {
    fn statusbar(&self) -> String {
        let mut temp = String::new();
        let temp_file_path = "/sys/class/hwmon/hwmon0/temp1_input";
        File::open(&temp_file_path)
            .unwrap_or_else(|_| panic!("Can not open file {}", temp_file_path))
            .read_to_string(&mut temp)
            .unwrap();
        temp = format!("+{:.2}.0°C", &temp[..2]);

        let lang = self.kbd_layout().to_string().to_uppercase();

        let datetime = Local::now();
        let date = &datetime.format("%d.%m.%y");
        let time = &datetime.format("%H:%M:%S");

        format!("    | {temp} | {lang} | {date} | {time} |   ")
    }
}

fn main() -> Result<(), &'static str> {
    let mut is_looped = false;
    let mut refresh_rate: u64 = 1000;
    {
        let mut ap = ArgumentParser::new();
        ap.refer(&mut is_looped)
            .add_option(&["-l", "--loop"], StoreTrue, "Loop program.");
        ap.refer(&mut refresh_rate).add_option(
            &["-r", "--refresh-rate"],
            Store,
            "Set refresh rate in milliseconds (default: 1000).",
        );
        ap.parse_args_or_exit();
    }

    let bar = X11Bar::new()?;
    if refresh_rate == 0 {
        return Err("Refresh-rate must be greater than 0");
    }
    bar.run(is_looped, Duration::from_millis(refresh_rate));

    Ok(())
}
