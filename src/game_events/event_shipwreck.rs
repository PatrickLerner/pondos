use crate::{
    game_events::{GameEventTriggerEventName, TriggerEvent},
    player::PlayerShipwreckEvent,
};
use bevy::prelude::*;

pub fn event_shipwreck(
    mut events: EventReader<PlayerShipwreckEvent>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    for _ in events.iter() {
        triggers.send(TriggerEvent {
            event: GameEventTriggerEventName::Shipwreck,
            scope: None,
        });
    }
}
