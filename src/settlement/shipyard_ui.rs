use super::{
    SelectedBuilding, Shipyard, MAX_HEIGHT, MAX_WIDTH, WINDOW_PADDING_X, WINDOW_PADDING_Y,
};
use crate::{
    game_state::GameState,
    player::{Player, TransportType},
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Align2},
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

            let window = windows.get_primary().unwrap();
            let win_max_width = window.width() - WINDOW_PADDING_X;
            let width = f32::min(win_max_width, MAX_WIDTH);
            let win_max_height = window.height() - WINDOW_PADDING_Y;
            let height = f32::min(win_max_height, MAX_HEIGHT);

            egui::Window::new("Shipyard")
                .anchor(Align2::CENTER_CENTER, (0., 0.))
                .resizable(false)
                .open(&mut open)
                .collapsible(false)
                .show(egui_context.ctx_mut(), |ui| {
                    ui.set_height(height);
                    ui.set_width(width);

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
                });

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
