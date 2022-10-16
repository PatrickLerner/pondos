use crate::{
    game_events::{GameEventTriggerEventName, TriggerEvent},
    settlement::{Settlement, VisitSettlementEvent},
};
use bevy::prelude::*;

pub fn event_visit_settlement(
    mut events: EventReader<VisitSettlementEvent>,
    mut triggers: EventWriter<TriggerEvent>,
    settlements: Query<&Settlement>,
) {
    for event in events.iter() {
        let settlement = settlements.get(event.settlement).unwrap();

        triggers.send(TriggerEvent {
            event: GameEventTriggerEventName::Settlement,
            scope: Some(settlement.name.to_owned()),
        });
    }
}
