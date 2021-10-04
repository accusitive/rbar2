use chrono::{DateTime, Local};

use crate::{TXType, Widget, WidgetType};

pub struct Clock(pub TXType);

impl<'a> Widget<'a> for Clock {
    type State = ();
    fn update(_: &mut Self::State) -> String {
        let local: DateTime<Local> = Local::now();
        local.format("📅 %a %b %e %I:%M:%S %P %Y").to_string()
    }

    fn get_delta() -> u64 {
        1000
    }

    fn get_name() -> String {
        "Clock".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }

    fn get_widget_type() -> WidgetType {
        WidgetType::Clock
    }
}
