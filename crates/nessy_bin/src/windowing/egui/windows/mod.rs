use egui::Ui;

pub mod game_window;
pub mod root_window;

pub trait EGuiWindow {
    fn render(ctx: &mut Ui);
}
