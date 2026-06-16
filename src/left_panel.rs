use bevy_egui::*;
use egui::SidePanel;

pub fn left_panel(mut contexts: EguiContexts) {
    // Get the egui context (unwrap if you're sure it exists)
    let ctx = contexts.ctx_mut().unwrap();

    SidePanel::left("left_panel")
        .resizable(true)
        // Give it a semi‑transparent background so it's visible
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 180)))
        .show(ctx, |ui| {
            ui.label("Left resizable panel");
            // Add more widgets here
        });
}
