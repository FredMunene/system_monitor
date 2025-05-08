use crate::ui::Window;
use imgui::Ui;

pub struct CpuWindow {
    // TODO: Add CPU window fields
}

impl CpuWindow {
    pub fn new() -> Self {
        Self {}
    }
}

impl Window for CpuWindow {
    fn render(&mut self, ui: &Ui) {
        ui.window("CPU Usage")
            .size([400.0, 300.0], imgui::Condition::FirstUseEver)
            .build(|| {
                ui.text("CPU Usage Graph");
                // TODO: Add CPU usage graph
            });
    }
} 