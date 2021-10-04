use chrono::{DateTime, Local};

use crate::{Widget, WidgetType, TXType};

pub struct Clock(pub TXType);

impl<'a> Widget<'a> for Clock {
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

    fn get_widget_type() -> WidgetType {
        WidgetType::Clock
    }
}