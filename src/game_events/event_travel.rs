use crate::{
    game_events::{GameEventTriggerEventName, TriggerEvent},
    player::PlayerTravelEvent,
};
use bevy::prelude::*;

pub fn event_travel(
    mut events: EventReader<PlayerTravelEvent>,
    mut triggers: EventWriter<TriggerEvent>,
) {
    for _ in events.iter() {
        triggers.send(TriggerEvent {
            event: GameEventTriggerEventName::Travel,
            scope: None,
        });
    }
}
