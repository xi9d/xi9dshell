use eframe::egui;
use crate::ui_style::Colors;
use crate::shell_state::ShellState;

/*pub fn render_header(ui: &mut egui::Ui, current_dir: &str) {
    ui.add_space(10.0);
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        ui.label(
            egui::RichText::new("❯")
                .color(Colors::CYAN)
                .size(24.0)
        );
        ui.label(
            egui::RichText::new(current_dir)
                .color(Colors::GREEN)
                .size(16.0)
        );
    });
    ui.add_space(5.0);
    ui.separator();
    ui.add_space(10.0);
}*/

pub fn render_output_area(ui: &mut egui::Ui, output: &[String]) {
    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .stick_to_bottom(false)
        .show(ui, |ui| {
            ui.add_space(5.0);
            for line in output {
                render_output_line(ui, line);
               // render_input_area(ui, "Xi9d>");
            }
        });
}

fn render_output_line(ui: &mut egui::Ui, line: &str) {
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        let (color, text) = if line.starts_with("Xi9d>") {
            (Colors::PURPLE, line)
        } else if line.starts_with("Error:") {
            (Colors::RED, line)
        } else {
            (Colors::OFF_WHITE, line)
        };
        
        ui.label(
            egui::RichText::new(text)
                .color(color)
                .monospace()
        );
    });
}

pub fn render_input_area(ui: &mut egui::Ui, state: &mut ShellState) -> bool {
    ui.add_space(10.0);

    let mut command_executed = false;

    ui.horizontal(|ui| {
        ui.add_space(10.0);
        
        ui.label(
            egui::RichText::new("Xi9d:>")
                .color(Colors::OFF_WHITE)
                .size(20.0)
        );
        
        ui.add_space(5.0);
        
        let text_edit = egui::TextEdit::singleline(&mut state.input)
            .desired_width(f32::INFINITY)
            .font(egui::TextStyle::Monospace)
            .text_color(Colors::BLACK);
        
        let response = ui.add(text_edit.frame(true));
        
        if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
            command_executed = handle_command_input(state);
            response.request_focus();
        }
        
        ui.add_space(10.0);
    });

    command_executed
}

/*pub fn render_toolbar(ui: &mut egui::Ui, state: &mut ShellState) {
    ui.add_space(5.0);
    ui.horizontal(|ui| {
        ui.add_space(10.0);
        
        if ui.button(
            egui::RichText::new("Clear")
                .color(Colors::BLACK)
        ).clicked() {
            state.clear_output();
        }
        
        ui.add_space(5.0);
        
        if ui.button(
            egui::RichText::new("Exit")
                .color(Colors::BLACK)
        ).clicked() {
            std::process::exit(0);
        }
    });
    ui.add_space(10.0);
}*/

fn handle_command_input(state: &mut ShellState) -> bool {
    let command = state.input.trim().to_string(); // so this removes any whitespace arount the
    // command
    if command.is_empty() {
        return false;
    }

    if command == "exit" || command == "quit" {
        std::process::exit(0);
    } else if command == "clear" {
        state.clear_output();
    } else {
        state.execute_command(command);
    }
    
    state.input.clear();
    return true
}
