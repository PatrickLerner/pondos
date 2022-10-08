use super::{CloseSettlementUIEvent, SelectedSettlement, Settlement};
use crate::{game_state::GameState, player::PlayerTravelEvent};
use bevy::prelude::*;
use bevy_egui::{
    egui::{self, Align, Align2},
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
                ui.set_height(30.);
                ui.set_width(180.);
                ui.with_layout(egui::Layout::right_to_left(Align::Max), |ui| {
                    ui.horizontal(|ui| {
                        if ui
                            .add_sized([80., 30.], egui::Button::new("Travel"))
                            .clicked()
                        {
                            handle_travel.send(PlayerTravelEvent::new(
                                entity.0,
                                settlement.position.x,
                                settlement.position.y,
                            ));
                            game_state.set(GameState::Settlement).unwrap();
                        }
                        if ui
                            .add_sized([60., 30.], egui::Button::new("Abort"))
                            .clicked()
                        {
                            events.send(CloseSettlementUIEvent);
                        }
                    });
                });
            });
    }
}
