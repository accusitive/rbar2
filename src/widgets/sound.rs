use crate::{Widget, TXType, utils::audio, WidgetType};

pub struct VolumeLevel(pub TXType);


impl<'a> Widget<'a> for VolumeLevel {
    fn update() -> String {
        let default_sink = audio::get_default_sink();
        let (h, s) = match default_sink.trim() {
            speakers!() => (" 🎧 ", "[ 🔊 ]"),
            headphones!() => ("[ 🎧 ]", " 🔊 "),
            _ => ("🎧", "🔊"),
        };
        let f = format!(
            "{} {} {} {}",
            h,
            audio::get_headphones_volume(),
            s,
            audio::get_speakers_volume()
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
    fn get_widget_type() -> WidgetType {
        WidgetType::Audio
    }
}