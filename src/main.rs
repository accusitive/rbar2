use std::{
    sync::{mpsc::Sender, Arc},
};
#[macro_use]
mod utils;
mod widgets;

// use crate::utils::*;
// use crate::utils::audio::{get_default_sink, get_headphones_volume, get_speakers_volume};

trait Widget<'a> {
    fn get_delta(&self) -> u64;
    fn get_name(&self) -> String;
    fn get_tx(&'a self) -> &'a TXType;
    fn update() -> String;
    fn get_widget_type() -> WidgetType;
    fn run(&'a self) {
        let a = self.get_tx();
        let t = a.as_ref().clone();
        let duration = std::time::Duration::from_millis(self.get_delta());

        std::thread::spawn(move || loop {
            t.send((Self::get_widget_type(), Self::update())).unwrap();
            std::thread::sleep(duration);
        });
    }
}





#[derive(Debug)]
pub enum WidgetType {
    Audio,
    Clock,
    Weather,
}
type TXType = Arc<Sender<(WidgetType, String)>>;

fn main() {
    let display = unsafe { x11::xlib::XOpenDisplay(0 as *const i8) };
    let root = unsafe { x11::xlib::XRootWindow(display, 0) };
    let (tx, rx) = std::sync::mpsc::channel::<(WidgetType, String)>();

    widgets::Clock(Arc::new(tx.clone())).run();
    widgets::VolumeLevel(Arc::new(tx.clone())).run();
    widgets::Weather(Arc::new(tx.clone())).run();

    let (mut clock, mut audio, mut weather) = (String::new(), String::new(), String::new());
    let mut title;
    for r in rx.iter() {
        match r.0 {
            WidgetType::Audio => audio = r.1,
            WidgetType::Clock => clock = r.1,
            WidgetType::Weather => weather = r.1,
        }
        title = format!("{} {} {}", audio, clock, weather);
        title.push('\0');
        unsafe {
            x11::xlib::XStoreName(display, root, title.as_ptr() as *const i8);
            x11::xlib::XFlush(display);
        };
    }
}
