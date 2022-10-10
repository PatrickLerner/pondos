use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Response, Ui},
    EguiContext,
};
use egui::{Color32, FontFamily, FontId, TextStyle};

#[inline]
pub fn panel_heading() -> TextStyle {
    TextStyle::Name("PanelHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(panel_heading(), FontId::new(20.0, Proportional));
    ctx.set_style(style);
}

pub fn color_mode(mut egui_context: ResMut<EguiContext>) {
    configure_text_styles(egui_context.ctx_mut());
    egui_context.ctx_mut().set_visuals(egui::Visuals::light());
}

pub fn enabled_color(enabled: bool) -> Color32 {
    if enabled {
        Color32::BLACK
    } else {
        Color32::GRAY
    }
}

pub fn large_button(ui: &mut Ui, width: f32, label: &str) -> Response {
    ui.add_sized([width, 30.], egui::Button::new(label))
}
