use std::{
    sync::{mpsc::SyncSender, Arc},
    time::Instant,
};
#[macro_use]
mod utils;
mod widgets;

trait Widget<'a> {
    type State: Sync + Default;
    fn get_delta() -> u64;
    fn get_name() -> String;
    fn get_tx(&'a self) -> TXType;
    fn update(s: &mut Self::State) -> String;
    fn get_widget_type() -> WidgetType;
    fn sly(&'a self) {}

    fn run<'r>(self: &'a Self) -> String
    where
        Self: Sync,
    {
        let t = self.get_tx();
        let duration = std::time::Duration::from_millis(Self::get_delta());
        std::thread::spawn(move || {
            let mut d = Self::State::default();
            loop {
                t.send((Self::get_widget_type(), Self::update(&mut d)))
                    .unwrap();
                std::thread::sleep(duration);
            }
        });
        String::new()
    }
}

#[derive(Debug)]
pub enum WidgetType {
    Audio,
    Clock,
    Weather,
    Load,
    CpuTemp,
}
unsafe impl Send for WidgetType {}
type TXType = Arc<SyncSender<(WidgetType, String)>>;

fn main() {
    let display = unsafe { x11::xlib::XOpenDisplay(0 as *const i8) };
    let root = unsafe { x11::xlib::XRootWindow(display, 0) };
    let (tx, rx) = std::sync::mpsc::sync_channel(1024);
    let a = Arc::new(tx.clone());
    let mut clock = widgets::Clock(a).run();
    let mut audio = widgets::VolumeLevel(Arc::new(tx.clone())).run();
    let mut weather = widgets::Weather(Arc::new(tx.clone())).run();
    let mut load = widgets::LoadAvg(Arc::new(tx.clone())).run();
    let mut temp = widgets::CpuTemp(Arc::new(tx.clone())).run();

    // let (mut clock, mut audio, mut weather) = (String::new(), String::new(), String::new());
    let mut title;
    for r in rx.iter() {
        match r.0 {
            WidgetType::Audio => audio = r.1,
            WidgetType::Clock => clock = r.1,
            WidgetType::Weather => weather = r.1,
            WidgetType::Load => load = r.1,
            WidgetType::CpuTemp => temp = r.1,
            // _ => todo!(),
        }
        title = format!("{} {} {} {} {}", temp, load, audio, clock, weather);
        // title = format!("{}", clock);

        title.push('\0');
        unsafe {
            x11::xlib::XStoreName(display, root, title.as_ptr() as *const i8);
            x11::xlib::XFlush(display);
        };
    }
}
