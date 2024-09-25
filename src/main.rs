use std::{
	ptr,
	fs::File,
	io::Read,
	thread::sleep,
	time::Duration,
	mem::MaybeUninit,
	ffi::{CString, CStr, c_char, c_ushort, c_uint},
};
use x11::xlib;
use chrono::Local;

static ONE_SEC: Duration = Duration::from_secs(1);

#[derive(Debug, Clone, Copy)]
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

#[allow(non_camel_case_types)]
type XkbRF_VarDefsPtr = *mut _XkbRF_VarDefs;

#[allow(non_upper_case_globals)]
static XkbUseCoreKbd: c_uint = 0x0100;

extern "C" {
	fn XkbRF_GetNamesProp(
		_3: *mut xlib::Display,
		_2: *const c_char,
		_1: XkbRF_VarDefsPtr
	) -> bool;
}

fn xsetroot<T: AsRef<str>>(new_name: T) {
	let name = CString::new(new_name.as_ref()).unwrap();

	unsafe {
		let display = xlib::XOpenDisplay(ptr::null());
		let screen = xlib::XDefaultScreen(display);
		let window = xlib::XRootWindow(display, screen);

		xlib::XStoreName(display, window, name.as_ptr());

		xlib::XCloseDisplay(display);
	};
}

fn lang() -> String {
	let (kl, s) = unsafe {
		let display = xlib::XOpenDisplay(ptr::null());

		let mut state: MaybeUninit<xlib::_XkbStateRec> = MaybeUninit::uninit();
		let _ = xlib::XkbGetState(display, XkbUseCoreKbd, state.as_mut_ptr());

		let mut vd: MaybeUninit<_XkbRF_VarDefs> = MaybeUninit::uninit();
		let _ = XkbRF_GetNamesProp(display, ptr::null(), vd.as_mut_ptr());

		xlib::XCloseDisplay(display);

		(CStr::from_ptr(vd.assume_init().layout).to_str().unwrap(), state.assume_init().group)
	};

	kl.split(",")
		.collect::<Vec<&str>>()[s as usize]
		.to_string()
}

fn temp() -> String {
	let mut temp = String::new();
	File::open("/sys/class/hwmon/hwmon0/temp1_input")
		.unwrap()
		.read_to_string(&mut temp)
		.unwrap();

	format!("+{:.2}.0°C", &temp[..2])
}

fn dwm_statusbar() {
	let lang = lang().to_uppercase();

	let datetime = Local::now();
	let date = &datetime.format("%d.%m.%y");
	let time = &datetime.format("%H:%M:%S");

	let temp = temp();

	let bar = format!("    | {temp} | {lang} | {date} | {time} |   ");
	xsetroot(&bar);
}

fn cli(args: Vec<String>) {
	match args.len() {
		1 => dwm_statusbar(),
		2 => {
			match args[1].as_ref() {
				"-l" | "--loop" => {
					loop {
						dwm_statusbar();
						sleep(ONE_SEC);
					};
				}
				_ => panic!("Undifined flag"),
			};
		}
		_ => panic!("Needed only one or less flags"),
	};
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	cli(args);
}
