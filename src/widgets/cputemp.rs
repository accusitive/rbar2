use crate::{utils::bash, TXType, Widget, WidgetType};

pub struct CpuTemp(pub TXType);
pub struct CpuTempData {
    a: i32,
}
impl Default for CpuTempData {
    fn default() -> Self {
        Self { a: Default::default() }
    }
}
impl<'a> Widget<'a> for CpuTemp {
    type State = CpuTempData;

    fn update(_: &mut Self::State) -> String {
        // s.push('⏱');
        let sens = bash::exec("sensors");
        let plus_index = sens.find('+');

        // println!("{}", s2);
        // s
        // s2.to_string()
        String::new()
    }

    fn get_delta(&self) -> u64 {
        1000
    }

    fn get_name(&self) -> String {
        "VolumeLevel".to_string()
    }

    fn get_tx(&'a self) -> TXType {
        self.0.clone()
    }
    fn get_widget_type() -> WidgetType {
        WidgetType::CpuTemp
    }
}
