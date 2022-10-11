use super::{CloseSettlementUIEvent, SelectedBuilding, SelectedSettlement};
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn close_event_handler(
    mut events: EventReader<CloseSettlementUIEvent>,
    mut selected_settlement: ResMut<Option<SelectedSettlement>>,
    mut selected_building: ResMut<Option<SelectedBuilding>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in events.iter() {
        *selected_settlement = None;
        *selected_building = None;
        game_state.pop().unwrap();
    }
}
