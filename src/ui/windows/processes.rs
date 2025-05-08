use crate::ui::Window;
use imgui::Ui;

pub struct ProcessWindow {
    // TODO: Add process window fields
}

impl ProcessWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for ProcessWindow {
    fn render(&mut self, ui: &Ui) {
        ui.window("Processes")
            .size([600.0, 400.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Process List");
                // TODO: Add process table
            });
    }
} 