use eframe::egui::{self, Ui};

/// Draws the default empty view when no route is selected.
pub fn none(ui: &mut Ui) {
    ui.with_layout(
        egui::Layout::centered_and_justified(egui::Direction::TopDown),
        |ui| {
            ui.label("Empty");
        },
    );
}
