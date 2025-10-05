mod command_executor;
mod ui_style;
mod shell_state;
mod ui_components;

use eframe::egui;
use shell_state::ShellState;
use ui_style::Colors;

struct Xi9dShell {
    state: ShellState,
}

impl Default for Xi9dShell {
    fn default() -> Self {
        Self {
            state: ShellState::new(),
        }
    }
}

impl eframe::App for Xi9dShell {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ui_style::apply_dark_theme(ctx);
        self.state.update_from_receiver();
        
        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(Colors::BLACK))
            .show(ctx, |ui| {
                ui_components::render_header(ui, &self.state.current_dir);
                ui_components::render_output_area(ui, &self.state.output);
                ui_components::render_input_area(ui, &mut self.state);
                ui_components::render_toolbar(ui, &mut self.state);
            });
        
        ctx.request_repaint();
    }
}
fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 650.0])
            .with_decorations(true),
        ..Default::default()
    };
    
    eframe::run_native(
        "Xi9d Shell",
        options,
        Box::new(|_cc| Ok(Box::new(Xi9dShell::default()))),
    )
}