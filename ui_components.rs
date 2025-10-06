use eframe::egui;
use crate::ui_style::Colors;
use crate::shell_state::ShellState;
use egui::{self, menu, RichText, Align, Layout}

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
        .auto_shrink([true, true])
        .stick_to_bottom(true)
        .show(ui, |ui| {
            ui.add_space(10.0);
            for line in output {
                render_output_line(ui, line);
               
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

pub fn render_toolbar(ui: &mut egui::Ui, state: &mut ShellState) {
    use egui::Color32 as Colors;

    // Add vertical padding
    ui.add_space(5.0);

    // The top menu bar
    egui::menu::bar(ui, |ui| {
        // ===== FILE =====
        ui.menu_button(RichText::new("📁 File").color(Colors::BLACK), |ui| {
            if ui.button("🆕 New File").clicked() {
                // TODO: Add new file logic
                ui.close_menu();
            }
            if ui.button("💾 Save").clicked() {
                // TODO: Add save logic
                ui.close_menu();
            }
            if ui.button("🚪 Exit").clicked() {
                state.confirm_exit = true;
                ui.close_menu();
            }
        });

        // ===== EDIT =====
        ui.menu_button(RichText::new("✏️ Edit").color(Colors::BLACK), |ui| {
            if ui.button("Undo").clicked() {
                // TODO
                ui.close_menu();
            }
            if ui.button("Redo").clicked() {
                // TODO
                ui.close_menu();
            }
            ui.separator();
            if ui.button("Cut").clicked() {
                // TODO
                ui.close_menu();
            }
            if ui.button("Copy").clicked() {
                // TODO
                ui.close_menu();
            }
            if ui.button("Paste").clicked() {
                // TODO
                ui.close_menu();
            }
        });

        // ===== VIEW =====
        ui.menu_button(RichText::new("👁️ View").color(Colors::BLACK), |ui| {
            if ui
                .checkbox(&mut state.show_output, "Show Output Panel")
                .clicked()
            {
                ui.close_menu();
            }

            if ui
                .checkbox(&mut state.dark_mode, "Dark Mode")
                .clicked()
            {
                // TODO: Apply theme switching logic
                ui.close_menu();
            }
        });

        // ===== ACTIONS =====
        ui.menu_button(RichText::new("⚙️ Actions").color(Colors::BLACK), |ui| {
            if ui.button("Clear Output").clicked() {
                // Example: call your existing clear_output()
                // state.clear_output();
                ui.close_menu();
            }
            if ui.button("Run Command").clicked() {
                // TODO
                ui.close_menu();
            }
        });

        // ===== HELP =====
        ui.menu_button(RichText::new("❓ Help").color(Colors::BLACK), |ui| {
            if ui.button("About").clicked() {
                // TODO: Show about dialog
                ui.close_menu();
            }
            if ui.button("Documentation").clicked() {
                // You could open a web link
                if let Err(e) = webbrowser::open("https://docs.rs/egui") {
                    eprintln!("Failed to open browser: {:?}", e);
                }
                ui.close_menu();
            }
        });
    });

    ui.add_space(10.0);

    // ===== CONFIRM EXIT MODAL =====
    if state.confirm_exit {
        egui::Window::new("Confirm Exit")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .show(ui.ctx(), |ui| {
                ui.label("Are you sure you want to exit?");
                ui.horizontal(|ui| {
                    if ui.button("Yes").clicked() {
                        std::process::exit(0);
                    }
                    if ui.button("Cancel").clicked() {
                        state.confirm_exit = false;
                    }
                });
            });
    }
}


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
