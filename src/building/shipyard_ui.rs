use crate::{
    building::Shipyard,
    game_state::{GameState, SettlementState},
    player::{Player, Ship, ShipSize},
    ui::{create_window, enabled_color, large_button, SelectedBuilding},
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

use super::ShipyardTask;

pub struct ShipTextures {
    #[allow(dead_code)]
    references: Vec<Handle<Image>>,
    small: egui::TextureId,
    medium: egui::TextureId,
    large: egui::TextureId,
}

impl ShipTextures {
    fn transport_type_image(&self, ship_size: ShipSize) -> egui::TextureId {
        match ship_size {
            ShipSize::Small => self.small,
            ShipSize::Medium => self.medium,
            ShipSize::Large => self.large,
        }
    }
}

pub fn shipyard_ui(
    mut shipyards: Query<&mut Shipyard>,
    mut ship_textures: Local<Option<ShipTextures>>,
    ui_resources: (Res<AssetServer>, Res<Windows>, ResMut<EguiContext>),
    resources: (
        ResMut<Player>,
        ResMut<State<GameState>>,
        Option<Res<SelectedBuilding>>,
    ),
) {
    let (asset_server, windows, mut egui_context) = ui_resources;
    let (mut player, mut game_state, selected_building) = resources;

    if ship_textures.is_none() {
        let small = asset_server.load("images/ship_small.png");
        let medium = asset_server.load("images/ship_medium.png");
        let large = asset_server.load("images/ship_large.png");

        let textures = ShipTextures {
            small: egui_context.add_image(small.clone_weak()),
            medium: egui_context.add_image(medium.clone_weak()),
            large: egui_context.add_image(large.clone_weak()),
            references: vec![small, medium, large],
        };
        *ship_textures = Some(textures);
    }

    if let Some(entity) = selected_building.as_ref() {
        if let Ok(mut shipyard) = shipyards.get_mut(entity.0) {
            let mut open = true;

            let ship_textures = ship_textures.as_ref().unwrap();

            create_window(
                egui_context.ctx_mut(),
                &windows,
                "Shipyard",
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

                    if let Some(task) = shipyard.task.clone() {
                        let ship_size = match task {
                            ShipyardTask::Construction(size) => size,
                            ShipyardTask::Repair(ship) => ship.size,
                        };

                        ui.with_layout(
                            egui::Layout::from_main_dir_and_cross_align(
                                egui::Direction::TopDown,
                                egui::Align::Center,
                            ),
                            |ui| {
                                ui.add(egui::widgets::Image::new(
                                    ship_textures.transport_type_image(ship_size),
                                    [250.0, 200.0],
                                ));
                                ui.heading(match task {
                                    ShipyardTask::Construction(size) => {
                                        format!("Construction of {}", Ship::new(size))
                                    }
                                    ShipyardTask::Repair(ship) => format!("Repair of {}", ship),
                                });
                                ui.add_space(5.);

                                if shipyard.task_time_remaining == 0 {
                                    ui.label("Ready for pickup");
                                    ui.add_space(10.);

                                    if large_button(ui, 200., "Add to convoy").clicked() {
                                        shipyard.task = None;
                                        player.convoy.push(match task {
                                            ShipyardTask::Construction(size) => Ship::new(size),
                                            ShipyardTask::Repair(ship) => ship.clone(),
                                        });
                                    }
                                } else if shipyard.task_time_remaining == 1 {
                                    ui.label("In progress for one more seasons");
                                } else {
                                    ui.label(format!(
                                        "In progress for {} more seasons",
                                        shipyard.task_time_remaining
                                    ));
                                }
                            },
                        );
                    } else {
                        ui.add_space(25.);
                        ui.columns(3, |columns| {
                            for (index, ship_size) in
                                vec![ShipSize::Small, ShipSize::Medium, ShipSize::Large]
                                    .into_iter()
                                    .enumerate()
                            {
                                let ship = Ship::new(ship_size);
                                let enabled = player.silver >= ship.price();

                                columns[index].with_layout(
                                    egui::Layout::from_main_dir_and_cross_align(
                                        egui::Direction::TopDown,
                                        egui::Align::Center,
                                    ),
                                    |ui| {
                                        ui.add(egui::widgets::Image::new(
                                            ship_textures.transport_type_image(ship_size),
                                            [100.0, 80.0],
                                        ));
                                        ui.heading(format!("{}", ship));
                                        ui.label(format!("{} {}", ship.price(), COIN_NAME));
                                    },
                                );

                                columns[index].add_space(5.);

                                let button = columns[index].add_sized(
                                    [100., 30.],
                                    egui::Button::new(
                                        RichText::new("Construct").color(enabled_color(enabled)),
                                    ),
                                );
                                if button.clicked() && enabled {
                                    player.silver -= ship.price();
                                    shipyard.task = Some(ShipyardTask::Construction(ship_size));
                                    shipyard.task_time_remaining = ship.construction_time();
                                }
                            }
                        });
                    }

                    let damaged_ships: Vec<(usize, Ship)> = player
                        .convoy
                        .clone()
                        .into_iter()
                        .enumerate()
                        .filter(|(_, ship)| ship.health() < 1.)
                        .collect();

                    if !damaged_ships.is_empty() {
                        ui.add_space(10.);
                        ui.heading("Damaged ships in your convoy");
                        ui.add_space(5.);

                        for (index, ship) in damaged_ships {
                            ui.horizontal(|ui| {
                                ui.label(format!(" - {}", ship));
                                let price = ship.repair_price();
                                let can_afford = player.silver >= price;
                                let can_store_resources =
                                    player.resource_space_left() >= ship.resource_space();
                                let enabled = can_afford && can_store_resources;

                                let button = ui.button(
                                    RichText::new(format!("Repair ({} {})", price, COIN_NAME))
                                        .color(enabled_color(enabled)),
                                );

                                if button.clicked() && enabled {
                                    player.silver -= price;
                                    let mut ship = player.convoy.remove(index);
                                    shipyard.task_time_remaining = ship.repair_time();
                                    ship.damage = 0;
                                    shipyard.task = Some(ShipyardTask::Repair(ship));
                                }

                                if !can_store_resources {
                                    ui.label(
                                        "ship is used to store resources and cannot be repaired",
                                    );
                                }
                            });
                        }
                    }
                },
            );

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
