pub mod windows;
pub mod widgets;

use imgui::Ui;

pub trait Window {
    fn render(&mut self, ui: &Ui);
}

pub trait Widget {
    fn render(&mut self, ui: &Ui);
} 