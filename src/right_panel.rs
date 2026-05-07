use bevy::prelude::*;
use bevy_egui::*;

pub fn right_panel(
    mut contexts: EguiContexts,
    mut camera: Single<&mut Camera, Without<EguiContext>>,
    window: Single<&mut Window>,
    ) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut side_panel = egui::SidePanel::right("right_panel")
        .resizable(false)
        .show(ctx, |ui| {
            ui.label("panel yay");
            ui.allocate_rect(ui.available_rect_before_wrap(), egui::Sense::hover());
        })
        .response
        .rect
        .width();

    Ok(())
}
