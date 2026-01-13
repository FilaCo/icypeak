use iced::window;

use crate::app::App;

impl App {
    pub fn title(&self, _: window::Id) -> String {
        String::from("IcyPeak")
    }
}
