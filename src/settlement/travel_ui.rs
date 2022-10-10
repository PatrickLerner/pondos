use super::{ui::production_ui, CloseSettlementUIEvent, SelectedSettlement, Settlement};
use crate::{
    game_state::{GameState, SettlementState},
    player::PlayerTravelEvent,
    ui_config::large_button,
};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align2},
    EguiContext,
};

pub fn travel_ui(
    mut egui_context: ResMut<EguiContext>,
    selected_settlement: Res<Option<SelectedSettlement>>,
    settlements: Query<&Settlement>,
    mut events: EventWriter<CloseSettlementUIEvent>,
    mut handle_travel: EventWriter<PlayerTravelEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    if let Some(entity) = selected_settlement.as_ref() {
        let settlement = settlements
            .get(entity.0)
            .expect("Expected settlement to be selected");

        egui::Window::new(format!("Travel to {}", settlement.name))
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, (0., 0.))
            .show(egui_context.ctx_mut(), |ui| {
                ui.add_space(10.);
                production_ui(ui, settlement);
                ui.add_space(10.);

                ui.columns(2, |columns| {
                    if large_button(&mut columns[0], 60., "Abort").clicked() {
                        events.send(CloseSettlementUIEvent);
                    }

                    if large_button(&mut columns[1], 80., "Travel").clicked() {
                        handle_travel.send(PlayerTravelEvent::new(
                            entity.0,
                            settlement.position.x,
                            settlement.position.y,
                        ));
                        game_state
                            .set(GameState::Settlement(SettlementState::Overview))
                            .unwrap();
                    }
                });
            });
    }
}
