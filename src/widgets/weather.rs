use crate::{utils::bash, TXType, Widget, WidgetType};

pub struct Weather(pub TXType);
impl<'a> Widget<'a> for Weather {
    type State = ();

    fn update(_: &mut Self::State) -> String {
        // bash::exec(r#"curl -s "wttr.in/?format=1" | grep -o ".[0-9].*""#.to_string());
        let weather = bash::exec(r#"curl -s "wttr.in/?format=1""#);
        let weather = weather.trim();

        format!("{}", weather)
    }

    fn get_delta() -> u64 {
        10 * 10000
    }

    fn get_name() -> String {
        "Weather".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }

    fn get_widget_type() -> WidgetType {
        WidgetType::Weather
    }
}
