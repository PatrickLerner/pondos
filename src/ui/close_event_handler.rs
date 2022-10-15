use super::{CloseSettlementUIEvent, SelectedBuilding, SelectedSettlement};
use crate::game_state::GameState;
use bevy::prelude::*;

pub fn close_event_handler(
    mut commands: Commands,
    mut events: EventReader<CloseSettlementUIEvent>,
    mut game_state: ResMut<State<GameState>>,
) {
    for _ in events.iter() {
        commands.remove_resource::<SelectedBuilding>();
        commands.remove_resource::<SelectedSettlement>();
        game_state.overwrite_set(GameState::Map).unwrap();
    }
}
