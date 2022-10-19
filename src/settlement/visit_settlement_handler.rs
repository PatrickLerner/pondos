use crate::{
    game_state::{GameState, SettlementState},
    settlement::VisitSettlementEvent,
    types::Settlement,
};
use bevy::prelude::*;

pub fn visit_settlement_handler(
    mut events: EventReader<VisitSettlementEvent>,
    mut game_state: ResMut<State<GameState>>,
    settlements: Query<&Settlement>,
) {
    for event in events.iter() {
        let settlement = settlements.get(event.settlement).unwrap();
        log::info!("Open settlement {}", settlement.name);
        game_state
            .push(GameState::Settlement(SettlementState::Overview))
            .unwrap();
    }
}
