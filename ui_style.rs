use eframe::egui;

pub struct Colors;

impl Colors {
    pub const BLACK: egui::Color32 = egui::Color32::BLACK;
    pub const CYAN: egui::Color32 = egui::Color32::from_rgb(139, 233, 253);
    pub const GREEN: egui::Color32 = egui::Color32::from_rgb(80, 250, 123);
    pub const PURPLE: egui::Color32 = egui::Color32::from_rgb(189, 147, 249);
    pub const RED: egui::Color32 = egui::Color32::from_rgb(255, 85, 85);
    pub const OFF_WHITE: egui::Color32 = egui::Color32::from_rgb(248, 248, 242);
    pub const YELLOW: egui::Color32 = egui::Color32::from_rgb(241, 250, 140);
    pub const PINK: egui::Color32 = egui::Color32::from_rgb(255, 121, 198);
    pub const DARK_GRAY: egui::Color32 = egui::Color32::from_rgb(30, 30, 30);
    pub const MEDIUM_GRAY: egui::Color32 = egui::Color32::from_rgb(40, 40, 40);
    pub const LIGHT_GRAY: egui::Color32 = egui::Color32::from_rgb(50, 50, 50);
}

pub fn apply_dark_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals = egui::Visuals::dark();
    style.visuals.window_fill = Colors::BLACK;
    style.visuals.panel_fill = Colors::BLACK;
    style.visuals.faint_bg_color = egui::Color32::from_rgb(10, 10, 10);
    style.visuals.extreme_bg_color = Colors::BLACK;
    
    style.visuals.widgets.inactive.bg_fill = Colors::DARK_GRAY;
    style.visuals.widgets.hovered.bg_fill = Colors::MEDIUM_GRAY;
    style.visuals.widgets.active.bg_fill = Colors::LIGHT_GRAY;
    
    ctx.set_style(style);
}

