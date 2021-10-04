use crate::{TXType, Widget, WidgetType};

pub struct LoadAvg(pub TXType);

impl<'a> Widget<'a> for LoadAvg {
    type State = ();

    fn update(_: &mut Self::State) -> String {
        let load = std::fs::read_to_string("/proc/loadavg").unwrap();
        let mut s = String::with_capacity(32);
        s.push('⏱');
        s.push(' ');

        s.push_str(&load.split_inclusive(' ').take(3).collect::<String>());
        s
    }

    fn get_delta(&self) -> u64 {
        1000
    }

    fn get_name(&self) -> String {
        "LoadAvg".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }
    fn get_widget_type() -> WidgetType {
        WidgetType::Load
    }
}
