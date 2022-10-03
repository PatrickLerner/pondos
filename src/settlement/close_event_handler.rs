use super::GameState;
use super::{CloseSettlementUIEvent, SelectedSettlement};
use bevy::prelude::*;

pub fn close_event_handler(
    mut events: EventReader<CloseSettlementUIEvent>,
    mut selected_settlement: ResMut<Option<SelectedSettlement>>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in events.iter() {
        *selected_settlement = None;
        game_state.pop().unwrap();
    }
}
