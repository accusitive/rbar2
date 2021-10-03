use chrono::prelude::*;
use parking_lot::Mutex;
use std::{sync::Arc, time::Instant};
#[macro_use]
mod audio;
mod bash;

use crate::audio::{get_default_sink, get_headphones_volume, get_speakers_volume};
trait Component<'a> {
    fn update(&self);
    fn get_content(&self) -> Arc<Mutex<String>>;
    fn get_delta(&self) -> u64;
    fn get_name(&self) -> String;
}

struct Clock {
    inner: Arc<Mutex<String>>,
}

impl<'a> Component<'a> for Clock {
    fn update(&self) {
        let local: DateTime<Local> = Local::now();
        *self.inner.lock() = local.format("📅 %a %b %e %I:%M:%S %P %Y").to_string();
    }
    fn get_content(&self) -> Arc<Mutex<String>> {
        self.inner.clone()
    }

    fn get_delta(&self) -> u64 {
        1000
    }

    fn get_name(&self) -> String {
        "Clock".to_string()
    }
}
struct VolumeLevel {
    inner: Arc<Mutex<String>>,
}
impl<'a> Component<'a> for VolumeLevel {
    fn update(&self) {
        let default_sink = get_default_sink();
        let (h, s) = match default_sink.trim() {
            speakers!() => (" 🎧 ", "[ 🔊 ]"),
            headphones!() => ("[ 🎧 ]", " 🔊 "),
            _ => ("🎧", "🔊"),
        };
        let f = format!(
            "{} {} {} {}",
            h,
            get_headphones_volume(),
            s,
            get_speakers_volume()
        );
        {
            *self.inner.lock() = f;
        }
    }

    fn get_content(&self) -> Arc<Mutex<String>> {
        self.inner.clone()
    }

    fn get_delta(&self) -> u64 {
        2000
    }

    fn get_name(&self) -> String {
        "VolumeLevel".to_string()
    }
}
struct Weather {
    inner: Arc<Mutex<String>>,
}
impl<'a> Component<'a> for Weather {
    fn update(&self) {
        let weather =
            bash::exec(r#"curl -s "wttr.in/?format=1" | grep -o ".[0-9].*""#.to_string());
        let weather = weather.trim();
        println!("{:#?}", weather);
        {
            *self.inner.lock() = format!("🌡{}", weather)
        }
    }

    fn get_content(&self) -> Arc<Mutex<String>> {
        self.inner.clone()
    }

    fn get_delta(&self) -> u64 {
        10 * 10000
    }

    fn get_name(&self) -> String {
        "Weather".to_string()
    }
}
fn main() {
    let display = unsafe { x11::xlib::XOpenDisplay(0 as *const i8) };
    let root = unsafe { x11::xlib::XRootWindow(display, 0) };
    let mut title = String::with_capacity(256);
    let clock = Clock {
        inner: Arc::new(Mutex::new(String::new())),
    };

    let volume_level = VolumeLevel {
        inner: Arc::new(Mutex::new(String::new())),
    };
    let weather = Weather {
        inner: Arc::new(Mutex::new(String::new())),
    };
    let components: Vec<Arc<dyn Component + Send + Sync>> =
        vec![Arc::new(clock), Arc::new(volume_level), Arc::new(weather)];

    for component in components.clone() {
        let ac = Arc::new(component);

        let _t = std::thread::spawn(move || loop {
            {
                let now = Instant::now();
                ac.update();
                let e = now.elapsed().as_millis();
                println!("{} took {}ms", ac.get_name(), e);
            }
            std::thread::sleep(std::time::Duration::from_millis(ac.get_delta()));
        });
    }
    loop {
        // time

        title.clear();
        for component in components.clone() {
            title.push_str(&component.get_content().lock());
            title.push_str(" | ");
        }
        // title.push_str(&local.format("%a %b %e %I:%M:%S %P %Y").to_string());
        title.push('\0');
        // cpu temp

        unsafe { x11::xlib::XStoreName(display, root, title.as_ptr() as *const i8) };
        unsafe {
            x11::xlib::XFlush(display);
        }
        std::thread::sleep(std::time::Duration::from_millis(1000));
    }
}
