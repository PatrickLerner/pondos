use crate::player::Player;
use crate::COIN_NAME;
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_egui::{egui::Align2, EguiContext};

pub fn info_ui(
    mut egui_context: ResMut<EguiContext>,
    player: Option<ResMut<Player>>,
    mut convoy_open: Local<bool>,
    #[cfg(debug_assertions)] mut dev_open: Local<bool>,
) {
    #[allow(unused_mut)]
    if let Some(mut player) = player {
        egui::Window::new("Info")
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::LEFT_TOP, (14., 14.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("{} {}", player.silver, COIN_NAME));
                    if ui
                        .small_button(format!("Convoy ({})", player.convoy.len()))
                        .clicked()
                    {
                        *convoy_open = !*convoy_open;
                    }

                    #[cfg(debug_assertions)]
                    if ui.small_button("DEV").clicked() {
                        *dev_open = !*dev_open;
                    }
                });

                if *convoy_open {
                    for transport in &player.convoy {
                        ui.label(format!(" - {}", transport));
                    }
                }

                #[cfg(debug_assertions)]
                if *dev_open && ui.small_button("Add Money").clicked() {
                    player.silver += match player.silver {
                        0..=999 => 100,
                        1000..=9999 => 1000,
                        10000.. => 10000,
                    };
                }
            });
    }
}

const NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn show_game_version(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn().insert_bundle(
        TextBundle::from_section(
            format!("{} v{}", NAME, VERSION),
            TextStyle {
                font: asset_server.load("fonts/FiraMono-Medium.ttf"),
                font_size: 14.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                bottom: Val::Px(5.0),
                right: Val::Px(15.0),
                ..default()
            },
            ..default()
        }),
    );
}
