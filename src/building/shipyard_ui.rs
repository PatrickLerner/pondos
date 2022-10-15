use crate::{
    building::Shipyard,
    game_state::{GameState, SettlementState},
    player::{Player, TransportType},
    ui::{create_window, enabled_color, large_button, SelectedBuilding},
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

pub struct ShipTextures {
    #[allow(dead_code)]
    references: Vec<Handle<Image>>,
    small: egui::TextureId,
    medium: egui::TextureId,
    large: egui::TextureId,
}

impl ShipTextures {
    fn transport_type_image(&self, transport_type: TransportType) -> egui::TextureId {
        match transport_type {
            TransportType::SmallShip => self.small,
            TransportType::MediumShip => self.medium,
            TransportType::LargeShip => self.large,
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
        Res<Option<SelectedBuilding>>,
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

                    if let Some(transport_type) = shipyard.construction {
                        ui.with_layout(
                            egui::Layout::from_main_dir_and_cross_align(
                                egui::Direction::TopDown,
                                egui::Align::Center,
                            ),
                            |ui| {
                                ui.add(egui::widgets::Image::new(
                                    ship_textures.transport_type_image(transport_type),
                                    [250.0, 200.0],
                                ));
                                ui.heading(format!("{}", transport_type));
                                ui.add_space(5.);

                                if shipyard.construction_time == 0 {
                                    ui.label("Ready for pickup");
                                    ui.add_space(10.);

                                    if large_button(ui, 200., "Add to convoy").clicked() {
                                        shipyard.construction = None;
                                        player.convoy.push(transport_type);
                                    }
                                } else if shipyard.construction_time == 1 {
                                    ui.label("In construction for one more seasons");
                                } else {
                                    ui.label(format!(
                                        "In construction for {} more seasons",
                                        shipyard.construction_time
                                    ));
                                }
                            },
                        );
                    } else {
                        ui.add_space(25.);
                        ui.columns(3, |columns| {
                            for (index, transport_type) in vec![
                                TransportType::SmallShip,
                                TransportType::MediumShip,
                                TransportType::LargeShip,
                            ]
                            .into_iter()
                            .enumerate()
                            {
                                let enabled = player.silver >= transport_type.price();

                                columns[index].with_layout(
                                    egui::Layout::from_main_dir_and_cross_align(
                                        egui::Direction::TopDown,
                                        egui::Align::Center,
                                    ),
                                    |ui| {
                                        ui.add(egui::widgets::Image::new(
                                            ship_textures.transport_type_image(transport_type),
                                            [100.0, 80.0],
                                        ));
                                        ui.heading(format!("{}", transport_type));
                                        ui.label(format!(
                                            "{} {}",
                                            transport_type.price(),
                                            COIN_NAME
                                        ));
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
                                    player.silver -= transport_type.price();
                                    shipyard.construction = Some(transport_type);
                                    shipyard.construction_time = transport_type.construction_time();
                                }
                            }
                        });
                    }
                },
            );

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
