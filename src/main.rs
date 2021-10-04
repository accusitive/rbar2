use std::sync::{mpsc::SyncSender, Arc};
#[macro_use]
mod utils;
mod widgets;

trait Widget<'a> {
    type WState: Sync + Default;
    fn get_delta() -> u64;
    fn get_name() -> String;
    fn get_tx(&'a self) -> TXType;
    fn update(s: &mut Self::WState) -> Option<String>;
    fn get_widget_type() -> WidgetType;
    fn sly(&'a self) {}

    fn run<'r>(self: &'a Self) -> String
    where
        Self: Sync,
    {
        let t = self.get_tx();
        let duration = std::time::Duration::from_millis(Self::get_delta());
        std::thread::spawn(move || {
            let mut d = Self::WState::default();
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
    Crypto
}
unsafe impl Send for WidgetType {}
type TXType = Arc<SyncSender<(WidgetType, Option<String>)>>;

fn main() {
    let display = unsafe { x11::xlib::XOpenDisplay(0 as *const i8) };
    let root = unsafe { x11::xlib::XRootWindow(display, 0) };
    let (tx, rx) = std::sync::mpsc::sync_channel(1024);
    let mut clock = widgets::Clock(Arc::new(tx.clone())).run();
    let mut audio = widgets::VolumeLevel(Arc::new(tx.clone())).run();
    let mut weather = widgets::Weather(Arc::new(tx.clone())).run();
    let mut load = widgets::LoadAvg(Arc::new(tx.clone())).run();
    let mut temp = widgets::CpuTemp(Arc::new(tx.clone())).run();
    let mut crypto = widgets::Crypto(Arc::new(tx.clone())).run();

    let mut title;
    for r in rx.iter() {
        if let Some(s) = r.1 {
            match r.0 {
                WidgetType::Audio => audio = s,
                WidgetType::Clock => clock = s,
                WidgetType::Weather => weather = s,
                WidgetType::Load => load = s,
                WidgetType::CpuTemp => temp = s,
                WidgetType::Crypto => crypto = s,
                
            }
        } else {
            println!("One of the widgets returned an error. {:?}", r.0)
        }

        title = format!("{} {} {} {} {} {}", crypto, temp, load, audio, clock, weather);

        title.push('\0');
        unsafe {
            x11::xlib::XStoreName(display, root, title.as_ptr() as *const i8);
            x11::xlib::XFlush(display);
        };
    }
}
