use crate::ui::Window;
use imgui::Ui;

pub struct NetworkWindow {
    // TODO: Add network window fields
}

impl NetworkWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for NetworkWindow {
    fn render(&mut self, ui: &Ui) {
        ui.window("Network")
            .size([500.0, 300.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Network Activity");
                // TODO: Add network stats
            });
    }
} 