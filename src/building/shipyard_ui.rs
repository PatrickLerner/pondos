use super::{SelectedBuilding, Shipyard};
use crate::{
    create_window,
    game_state::{GameState, SettlementState},
    player::{Player, TransportType},
    ui_config::{enabled_color, large_button},
    COIN_NAME,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, RichText},
    EguiContext,
};

pub fn shipyard_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_building: Res<Option<SelectedBuilding>>,
    mut shipyards: Query<&mut Shipyard>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    mut player: ResMut<Player>,
) {
    if let Some(entity) = selected_building.as_ref() {
        if let Ok(mut shipyard) = shipyards.get_mut(entity.0) {
            let mut open = true;

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
                                .set(GameState::Settlement(SettlementState::Overview))
                                .unwrap()
                        }
                    });

                    if let Some(construction) = shipyard.construction {
                        if shipyard.construction_time == 0 {
                            ui.label(format!("{} ready for pickup", construction));

                            if large_button(ui, 100., "Pickup").clicked() {
                                shipyard.construction = None;
                                player.convoy.push(construction);
                            }
                        } else {
                            ui.label(format!(
                                "Constructing {} for {} seasons",
                                construction, shipyard.construction_time
                            ));
                        }
                    } else {
                        for transport_type in vec![
                            TransportType::SmallShip,
                            TransportType::MediumShip,
                            TransportType::LargeShip,
                        ]
                        .into_iter()
                        {
                            let enabled = player.silver >= transport_type.price();

                            let button = ui.add_sized(
                                [300., 30.],
                                egui::Button::new(
                                    RichText::new(format!(
                                        "Construct a {} ({} {})",
                                        transport_type,
                                        transport_type.price(),
                                        COIN_NAME
                                    ))
                                    .color(enabled_color(enabled)),
                                ),
                            );

                            if button.clicked() && enabled {
                                player.silver -= transport_type.price();
                                shipyard.construction = Some(transport_type);
                                shipyard.construction_time = 5;
                            }
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
