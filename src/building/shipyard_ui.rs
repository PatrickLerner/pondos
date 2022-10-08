use super::{SelectedBuilding, Shipyard};
use crate::{
    create_window,
    game_state::GameState,
    player::{Player, TransportType},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align},
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
                        let button =
                            ui.add_sized([100., 30.], egui::Button::new("Back to Overview"));
                        if button.clicked() {
                            game_state.pop().unwrap();
                        }
                    });

                    if let Some(construction) = shipyard.construction {
                        if shipyard.construction_time == 0 {
                            ui.label(format!("{:?} ready for pickup", construction));

                            if ui.button("Pickup").clicked() {
                                shipyard.construction = None;
                                player.convoy.push(construction);
                            }
                        } else {
                            ui.label(format!(
                                "Constructing {:?} for {} seasons",
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
                            if ui
                                .button(format!("Construct a {:?}", transport_type))
                                .clicked()
                            {
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
