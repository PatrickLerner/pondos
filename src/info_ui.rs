use crate::{types::Player, COIN_NAME};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

#[cfg(debug_assertions)]
#[derive(Default)]
pub struct DevState {
    open: bool,
    event_open: bool,
    event_name: String,
}

pub fn info_ui(
    mut egui_context: ResMut<EguiContext>,
    player: Option<ResMut<Player>>,
    mut convoy_open: Local<bool>,
    #[cfg(debug_assertions)] mut dev: Local<DevState>,
    #[cfg(debug_assertions)] mut trigger_event: EventWriter<
        crate::game_events::AddEventToCurrentEvent,
    >,
) {
    #[allow(unused_mut)]
    if let Some(mut player) = player {
        #[cfg(debug_assertions)]
        if dev.event_open {
            let mut triggered = false;
            let mut open = dev.event_open;
            egui::Window::new("Trigger Event")
                .resizable(false)
                .collapsible(false)
                .open(&mut open)
                .anchor(Align2::CENTER_CENTER, (0., 0.))
                .show(egui_context.ctx_mut(), |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Event id:");
                        ui.text_edit_singleline(&mut dev.event_name);
                        if ui.button("Launch").clicked() {
                            trigger_event.send(crate::game_events::AddEventToCurrentEvent::new(
                                dev.event_name.clone(),
                            ));
                            triggered = true;
                        }
                    });
                });

            if !open || triggered {
                dev.event_open = false;
                dev.event_name = "".to_owned();
            }
        }

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
                        dev.open = !dev.open;
                    }
                });

                if *convoy_open {
                    for transport in &player.convoy {
                        ui.label(if transport.damage > 0 {
                            format!(
                                " - {} ({}%)",
                                transport,
                                (transport.health() * 100.).floor()
                            )
                        } else {
                            format!(" - {}", transport)
                        });
                    }
                }

                #[cfg(debug_assertions)]
                if dev.open {
                    ui.add_space(5.);
                    if ui.small_button("Add Money").clicked() {
                        player.silver += match player.silver {
                            0..=999 => 100,
                            1000..=9999 => 1000,
                            10000.. => 10000,
                        };
                    }

                    if ui.small_button("Trigger Event").clicked() {
                        dev.event_open = !dev.event_open;
                        dev.event_name = "".to_owned();
                    }
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
