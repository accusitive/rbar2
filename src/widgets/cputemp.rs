use crate::{TXType, Widget, WidgetType};

pub struct CpuTemp(pub TXType);
impl<'a> Widget<'a> for CpuTemp {
    type WState = ();

    fn update(_: &mut Self::WState) -> Option<String> {
        let mut s = std::fs::read_to_string("/sys/class/hwmon/hwmon0/temp1_input").ok()?;
        s.insert(s.len() - 4, '.');
        Some(format!("{}°C", s))
    }

    fn get_delta() -> u64 {
        1000
    }

    fn get_name() -> String {
        "VolumeLevel".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }
    fn get_widget_type() -> WidgetType {
        WidgetType::CpuTemp
    }
}
