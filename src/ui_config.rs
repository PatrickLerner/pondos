use bevy::prelude::*;
use bevy_egui::{egui, EguiContext};
use egui::{FontFamily, FontId, TextStyle};

#[inline]
pub fn panel_heading() -> TextStyle {
    TextStyle::Name("PanelHeading".into())
}

pub fn configure_text_styles(ctx: &egui::Context) {
    use FontFamily::Proportional;

    let mut style = (*ctx.style()).clone();
    style
        .text_styles
        .insert(panel_heading(), FontId::new(40.0, Proportional));
    ctx.set_style(style);
}

pub fn color_mode(mut egui_context: ResMut<EguiContext>) {
    configure_text_styles(egui_context.ctx_mut());
    egui_context.ctx_mut().set_visuals(egui::Visuals::light());
}
