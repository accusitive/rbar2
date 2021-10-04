use chrono::prelude::*;
use std::{
    sync::{mpsc::Sender, Arc},
};
#[macro_use]
mod audio;
mod bash;

use crate::audio::{get_default_sink, get_headphones_volume, get_speakers_volume};

trait Component<'a> {
    fn get_delta(&self) -> u64;
    fn get_name(&self) -> String;
    fn get_tx(&'a self) -> &'a TXType;
    fn update() -> String;
    fn get_from() -> From;
    fn run(&'a self) {
        let a = self.get_tx();
        let t = a.as_ref().clone();
        let duration = std::time::Duration::from_millis(self.get_delta());

        std::thread::spawn(move || loop {
            t.send((Self::get_from(), Self::update())).unwrap();
            std::thread::sleep(duration);
        });
    }
}
struct Clock(TXType);
struct VolumeLevel(TXType);
struct Weather(TXType);
impl<'a> Component<'a> for Clock {
    fn update() -> String {
        let local: DateTime<Local> = Local::now();
        local.format("📅 %a %b %e %I:%M:%S %P %Y").to_string()
    }

    fn get_delta(&self) -> u64 {
        1000
    }

    fn get_name(&self) -> String {
        "Clock".to_string()
    }

    fn get_tx(&'a self) -> &'a TXType {
        &self.0
    }

    fn get_from() -> From {
        From::Clock
    }
}

impl<'a> Component<'a> for VolumeLevel {
    fn update() -> String {
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

        f
    }

    fn get_delta(&self) -> u64 {
        2000
    }

    fn get_name(&self) -> String {
        "VolumeLevel".to_string()
    }

    fn get_tx(&'a self) -> &'a TXType {
        &self.0
    }
    fn get_from() -> From {
        From::Audio
    }
}

impl<'a> Component<'a> for Weather {
    fn update() -> String {
        // bash::exec(r#"curl -s "wttr.in/?format=1" | grep -o ".[0-9].*""#.to_string());
        let weather = bash::exec(r#"curl -s "wttr.in/?format=1""#.to_string());
        let weather = weather.trim();

        format!("{}", weather)
    }

    fn get_delta(&self) -> u64 {
        10 * 10000
    }

    fn get_name(&self) -> String {
        "Weather".to_string()
    }

    fn get_tx(&'a self) -> &'a TXType {
        &self.0
    }

    fn get_from() -> From {
        From::Weather
    }
}

#[derive(Debug)]
enum From {
    Audio,
    Clock,
    Weather,
}
type TXType = Arc<Sender<(From, String)>>;

fn main() {
    let display = unsafe { x11::xlib::XOpenDisplay(0 as *const i8) };
    let root = unsafe { x11::xlib::XRootWindow(display, 0) };
    let (tx, rx) = std::sync::mpsc::channel::<(From, String)>();

    Clock(Arc::new(tx.clone())).run();
    VolumeLevel(Arc::new(tx.clone())).run();
    Weather(Arc::new(tx.clone())).run();

    let (mut clock, mut audio, mut weather) = (String::new(), String::new(), String::new());
    let mut title;
    for r in rx.iter() {
        match r.0 {
            From::Audio => audio = r.1,
            From::Clock => clock = r.1,
            From::Weather => weather = r.1,
        }
        title = format!("{} {} {}", audio, clock, weather);
        title.push('\0');
        unsafe {
            x11::xlib::XStoreName(display, root, title.as_ptr() as *const i8);
            x11::xlib::XFlush(display);
        };
    }
}
