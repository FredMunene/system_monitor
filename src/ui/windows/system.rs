use crate::ui::Window;
use imgui::Ui;

pub struct SystemWindow {
    // TODO: Add system window fields
}

impl SystemWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for SystemWindow {
    fn render(&mut self, ui: &Ui) {
        ui.window("System Overview")
            .size([300.0, 200.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("System Information");
                // TODO: Add system information display
            });
    }
} 