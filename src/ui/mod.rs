use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Response, Ui},
    EguiContext,
};
use egui::{Color32, FontFamily, FontId, TextStyle};

mod close_by_keyboard;
mod close_event_handler;

pub use close_by_keyboard::close_by_keyboard;
pub use close_event_handler::close_event_handler;

pub struct CloseSettlementUIEvent;

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedSettlement(pub Entity);

#[derive(Debug, PartialEq, Eq)]
pub struct SelectedBuilding(pub Entity);

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

const WINDOW_PADDING_X: f32 = 40.;
const WINDOW_PADDING_Y: f32 = 80.;
const MAX_WIDTH: f32 = 720.;
const MAX_HEIGHT: f32 = 720.;
const MOBILE_BREAK_POINT: f32 = 400.;

pub fn create_window<'a>(
    ctx: &bevy_egui::egui::Context,
    windows: &'a Windows,
    name: &str,
    open: &mut bool,
    add_contents: impl FnOnce(&mut bevy_egui::egui::Ui),
) {
    create_window_with_mobile(ctx, windows, name, open, |ui, _| add_contents(ui))
}

pub fn create_window_with_mobile<'a>(
    ctx: &bevy_egui::egui::Context,
    windows: &'a Windows,
    name: &str,
    open: &mut bool,
    add_contents: impl FnOnce(&mut bevy_egui::egui::Ui, bool),
) {
    let window = windows.get_primary().unwrap();
    let win_max_width = window.width() - WINDOW_PADDING_X;
    let width = f32::min(win_max_width, MAX_WIDTH);
    let win_max_height = window.height() - WINDOW_PADDING_Y;
    let height = f32::min(win_max_height, MAX_HEIGHT);

    bevy_egui::egui::Window::new(name)
        .anchor(bevy_egui::egui::Align2::CENTER_CENTER, (0., 0.))
        .resizable(false)
        .collapsible(false)
        .open(open)
        .show(ctx, |ui| {
            ui.set_width(width);
            ui.set_height(height);

            let mobile = win_max_width <= MOBILE_BREAK_POINT;
            add_contents(ui, mobile);
        });
}
