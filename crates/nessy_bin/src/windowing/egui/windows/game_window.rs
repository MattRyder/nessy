use egui::{Frame, Label, Ui, Window};

use crate::rendering::callback::TriangleCallback;
use crate::windowing::egui::windows::EGuiWindow;

pub struct GameWindow;

impl EGuiWindow for GameWindow {
    fn render(ctx: &mut Ui) {
        let size = [300.0, 300.0];
        let position = [ctx.available_width() * 0.5, ctx.available_height() * 0.5];

        Window::new("game_window")
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .fixed_pos(position)
            .min_size(size)
            .show(ctx, |ui| {
                ui.add(Label::new("NES Game goes here."));
                Frame::canvas(ui.style()).show(ui, Self::custom_painting);
            });
    }
}

impl GameWindow {
    pub fn custom_painting(ui: &mut egui::Ui) {
        let (rect, _response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());

        // self.angle += response.drag_motion().x * 0.01;
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            TriangleCallback { angle: 0.0 },
        ));
    }
}
