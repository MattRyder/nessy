use egui::{MenuBar, Panel, Ui, Window};

use crate::windowing::egui::windows::{EGuiWindow, game_window::GameWindow};

pub struct RootWindow;

impl EGuiWindow for RootWindow {
    fn render(ctx: &mut Ui) {
        Window::new("root_window")
            .title_bar(false)
            .resizable(false)
            .movable(false)
            .fixed_pos([0.0, 0.0])
            .min_width(ctx.available_width())
            .show(ctx, |ui| {
                Panel::top("menu_panel")
                    .show_separator_line(false)
                    .show(ui, |ui| {
                        MenuBar::new().ui(ui, |ui| {
                            ui.menu_button("File", |ui| {
                                if ui.button("Save").clicked() {
                                    //functionality
                                }
                                if ui.button("Quit").clicked() {
                                    std::process::exit(0);
                                }
                            });
                        });
                    });

                GameWindow::render(ui);
            });
    }
}
