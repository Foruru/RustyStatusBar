use argparse::{ArgumentParser, Store, StoreTrue};
use chrono::Local;
use std::path::PathBuf;
use std::{thread::sleep, time::Duration};

mod x11_bar;
pub mod x11_ffi;
use x11_bar::*;

struct App {
    x11_bar: X11Bar,
    hwmon_cpu_temp_file: PathBuf,
}

impl App {
    pub fn new() -> Result<Self, &'static str> {
        let hwmon_cpu_temp_file = std::fs::read_dir("/sys/class/hwmon")
            .map_err(|_| "Failed to read /sys/class/hwmon")?
            .flatten()
            .find(|entry| {
                std::fs::read_to_string(entry.path().join("name"))
                    .map(|name| matches!(name.trim(), "k10temp" | "coretemp"))
                    .unwrap_or(false)
            })
            .map(|entry| entry.path())
            .ok_or("No CPU temperature sensor found (k10temp/coretemp)")?
            .join("temp1_input");

        Ok(Self {
            x11_bar: X11Bar::new()?,
            hwmon_cpu_temp_file,
        })
    }

    pub fn statusbar(&self) -> String {
        let temp = std::fs::read_to_string(&self.hwmon_cpu_temp_file).unwrap();

        let lang = self.x11_bar.kbd_layout().to_string().to_uppercase();

        let datetime = Local::now();
        let date = &datetime.format("%d.%m.%y");
        let time = &datetime.format("%H:%M:%S");

        format!("    | +{temp:.2}.0°C | {lang} | {date} | {time} |   ")
    }

    pub fn run(&self, is_looped: bool, refresh_rate: Duration) {
        self.x11_bar.xsetroot(&self.statusbar());
        while is_looped {
            sleep(refresh_rate);
            self.x11_bar.xsetroot(&self.statusbar());
        }

        self.x11_bar.close_display();
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

    if refresh_rate == 0 {
        return Err("Refresh-rate must be greater than 0");
    }

    let app = App::new()?;
    app.run(is_looped, Duration::from_millis(refresh_rate));

    Ok(())
}
