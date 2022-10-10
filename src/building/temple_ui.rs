use super::{SelectedBuilding, Temple};
use crate::{
    create_window,
    game_state::{GameState, SettlementState},
    player::Player,
    ui_config::large_button,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align},
    EguiContext,
};

pub fn temple_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_building: Res<Option<SelectedBuilding>>,
    mut temples: Query<&mut Temple>,
    mut game_state: ResMut<State<GameState>>,
    windows: Res<Windows>,
    mut player: ResMut<Player>,
) {
    if let Some(entity) = selected_building.as_ref() {
        if let Ok(mut temple) = temples.get_mut(entity.0) {
            let mut open = true;

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
                                .set(GameState::Settlement(SettlementState::Overview))
                                .unwrap()
                        }
                    });
                },
            );

            if !open {
                game_state.pop().unwrap();
            }
        }
    }
}
