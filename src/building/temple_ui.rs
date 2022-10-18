use crate::{
    building::Temple,
    game_state::{GameState, SettlementState},
    types::Player,
    ui::{create_window, enabled_color, large_button, SelectedBuilding},
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

pub struct TempleTextures {
    #[allow(dead_code)]
    reference: Handle<Image>,
    image: egui::TextureId,
}

pub fn temple_ui(
    mut temples: Query<&mut Temple>,
    mut temple_textures: Local<Option<TempleTextures>>,
    ui_resources: (Res<AssetServer>, Res<Windows>, ResMut<EguiContext>),
    resources: (
        ResMut<Player>,
        ResMut<State<GameState>>,
        Option<Res<SelectedBuilding>>,
    ),
) {
    let (asset_server, windows, mut egui_context) = ui_resources;
    let (mut player, mut game_state, selected_building) = resources;

    if temple_textures.is_none() {
        let image = asset_server.load("images/temple.png");

        let textures = TempleTextures {
            image: egui_context.add_image(image.clone_weak()),
            reference: image,
        };
        *temple_textures = Some(textures);
    }

    if let Some(entity) = selected_building.as_ref() {
        if let Ok(mut temple) = temples.get_mut(entity.0) {
            let mut open = true;

            let temple_textures = temple_textures.as_ref().unwrap();

            create_window(
                egui_context.ctx_mut(),
                &windows,
                &format!("Temple of {}", temple.info.deity),
                &mut open,
                |ui| {
                    ui.add_space(10.);
                    ui.with_layout(egui::Layout::right_to_left(Align::Min), |ui| {
                        if large_button(ui, 100., "Back to Overview").clicked() {
                            game_state
                                .overwrite_set(GameState::Settlement(SettlementState::Overview))
                                .unwrap()
                        }
                    });

                    ui.with_layout(
                        egui::Layout::from_main_dir_and_cross_align(
                            egui::Direction::TopDown,
                            egui::Align::Center,
                        ),
                        |ui| {
                            ui.add(egui::widgets::Image::new(
                                temple_textures.image,
                                [250.0, 200.0],
                            ));
                            ui.heading(format!("Offerings to {}", temple.info.deity));
                            ui.add_space(5.);

                            {
                                if temple.offers_made > 0 {
                                    ui.add_space(5.);
                                    ui.label(format!(
                                        "Offers made: {} {}",
                                        temple.offers_made, COIN_NAME
                                    ));
                                    ui.add_space(5.);
                                }

                                ui.columns(3, |columns| {
                                    for (index, (name, price)) in
                                        vec![("Small", 10), ("Medium", 50), ("Large", 100)]
                                            .into_iter()
                                            .enumerate()
                                    {
                                        let enabled = player.silver >= price;
                                        let width = columns[index].available_width();
                                        let button = columns[index].add_sized(
                                            [width, 30.],
                                            egui::Button::new(
                                                RichText::new(format!(
                                                    "{} offering ({} {})",
                                                    name, price, COIN_NAME
                                                ))
                                                .color(enabled_color(enabled)),
                                            ),
                                        );

                                        if button.clicked() && enabled {
                                            player.silver -= price;
                                            temple.offers_made += price;
                                        }
                                    }
                                });
                            }

                            ui.add_space(10.);
                            ui.heading("Donations to temple");
                            ui.add_space(5.);

                            {
                                if temple.temple_donations_made > 0 {
                                    ui.add_space(5.);
                                    ui.label(format!(
                                        "Donations to temple made: {} {}",
                                        temple.temple_donations_made, COIN_NAME
                                    ));
                                    ui.add_space(5.);
                                }

                                ui.columns(3, |columns| {
                                    for (index, (name, price)) in
                                        vec![("Small", 10), ("Medium", 50), ("Large", 100)]
                                            .into_iter()
                                            .enumerate()
                                    {
                                        let enabled = player.silver >= price;
                                        let width = columns[index].available_width();
                                        let button = columns[index].add_sized(
                                            [width, 30.],
                                            egui::Button::new(
                                                RichText::new(format!(
                                                    "{} donation ({} {})",
                                                    name, price, COIN_NAME
                                                ))
                                                .color(enabled_color(enabled)),
                                            ),
                                        );

                                        if button.clicked() && enabled {
                                            player.silver -= price;
                                            temple.temple_donations_made += price;
                                        }
                                    }
                                });
                            }

                            ui.add_space(10.);
                            ui.heading("Donations to feed local people");
                            ui.add_space(5.);

                            {
                                if temple.poor_donations_made > 0 {
                                    ui.add_space(5.);
                                    ui.label(format!(
                                        "Donations to feed local people made: {} {}",
                                        temple.poor_donations_made, COIN_NAME
                                    ));
                                    ui.add_space(5.);
                                }

                                ui.columns(3, |columns| {
                                    for (index, (name, price)) in
                                        vec![("Small", 10), ("Medium", 50), ("Large", 100)]
                                            .into_iter()
                                            .enumerate()
                                    {
                                        let enabled = player.silver >= price;
                                        let width = columns[index].available_width();
                                        let button = columns[index].add_sized(
                                            [width, 30.],
                                            egui::Button::new(
                                                RichText::new(format!(
                                                    "{} donation ({} {})",
                                                    name, price, COIN_NAME
                                                ))
                                                .color(enabled_color(enabled)),
                                            ),
                                        );

                                        if button.clicked() && enabled {
                                            player.silver -= price;
                                            temple.poor_donations_made += price;
                                        }
                                    }
                                });
                            }
                        },
                    );
                },
            );

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
