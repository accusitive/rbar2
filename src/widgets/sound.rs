use crate::{utils::audio, TXType, Widget, WidgetType};

pub struct VolumeLevel(pub TXType);

impl<'a> Widget<'a> for VolumeLevel {
    type State = ();

    fn update(_: &mut Self::State) -> String {
        let default_sink = audio::get_default_sink();
        let (h, s) = match default_sink.trim() {
            speakers!() => (" 🎧 ", "[🔊]"),
            headphones!() => ("[🎧]", " 🔊 "),
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

    fn get_delta() -> u64 {
        500
    }

    fn get_name() -> String {
        "VolumeLevel".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }
    fn get_widget_type() -> WidgetType {
        WidgetType::Audio
    }
}
