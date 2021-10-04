use crate::{Widget, TXType, utils::bash, WidgetType};

pub struct Weather(pub TXType);
impl<'a> Widget<'a> for Weather {
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

    fn get_widget_type() -> WidgetType {
        WidgetType::Weather
    }
}