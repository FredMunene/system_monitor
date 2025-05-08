use crate::ui::Window;
use imgui::Ui;

pub struct MemoryWindow {
    // TODO: Add memory window fields
}

impl MemoryWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for MemoryWindow {
    fn render(&mut self, ui: &Ui) {
        ui.window("Memory Usage")
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("Memory Usage Graph");
                // TODO: Add memory usage graph
            });
    }
} 