use crate::ui::Widget;
use imgui::{Ui, TableFlags};

pub struct Table {
    label: String,
    columns: Vec<String>,
}

impl Table {
    pub fn new(label: &str, columns: Vec<&str>) -> Self {
        Self {
            label: label.to_string(),
            columns: columns.into_iter().map(String::from).collect(),
        }
    }
}

impl Widget for Table {
    fn render(&mut self, ui: &Ui) {
        if let Some(_table) = ui.begin_table(&self.label, self.columns.len()) {
            // Header
            for column in &self.columns {
                ui.table_setup_column(column);
            }
            ui.table_headers_row();

            // TODO: Implement table rows
            ui.table_next_row();
        }
    }
} 