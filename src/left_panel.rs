use bevy::prelude::Res;
use bevy_egui::*;
use egui::SidePanel;
use crate::algorithm_enum::PointAlgorithm;

pub fn left_panel(
    mut contexts: EguiContexts,
    point_algorithm: Res<PointAlgorithm>,
) {
    // Get the egui context (unwrap if you're sure it exists)
    let ctx = contexts.ctx_mut().unwrap();

    SidePanel::left("left_panel")
        // Give it a semi‑transparent background so it's visible
        .min_width(200.0)
        .frame(egui::Frame::none().fill(egui::Color32::from_rgba_premultiplied(0, 0, 0, 180)))
        .show(ctx, |ui| {
            ui.label(match point_algorithm.into_inner() {
                PointAlgorithm::Meh => {"No optimisation algorithm"}
                PointAlgorithm::PerpendicularDistance { .. } => {"Perpendicular distance algorithm"}
                PointAlgorithm::CTR { .. } => {"Cumulatiave triangle distance algorithm"}
                PointAlgorithm::MinimumDistance { .. } => {"Minimum distance algorithm"}
            })
            // Add more widgets here
        });
}
