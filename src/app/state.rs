use iced::Task;

use crate::app::Message;

#[derive(Debug)]
pub struct App {}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (Self {}, Task::none())
    }
}
