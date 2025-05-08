use crate::ui::Widget;
use imgui::Ui;

pub struct LineGraph {
    label: String,
    data: Vec<f32>,
    max_points: usize,
}

impl LineGraph {
    pub fn new(label: &str, max_points: usize) -> Self {
        Self {
            label: label.to_string(),
            data: Vec::with_capacity(max_points),
            max_points,
        }
    }

    pub fn add_point(&mut self, value: f32) {
        if self.data.len() >= self.max_points {
            self.data.remove(0);
        }
        self.data.push(value);
    }
}

impl Widget for LineGraph {
    fn render(&mut self, ui: &Ui) {
        // Create a simple line graph using a child window
        ui.group(|| {
            ui.text(&self.label);
            let graph_size = [ui.content_region_avail()[0], 100.0];

            if !self.data.is_empty() {
                let min = self.data.iter().fold(f32::INFINITY, |a, &b| a.min(b));
                let max = self.data.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

                ui.plot_lines(&self.label, &self.data)
                    .graph_size(graph_size)
                    .scale_min(min)
                    .scale_max(max)
                    .build();
            }
        });
    }
} 